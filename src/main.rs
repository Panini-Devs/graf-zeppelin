use std::collections::HashMap;
use std::env;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use serenity::framework::StandardFramework;
use serenity::framework::standard::Configuration;
use serenity::framework::standard::macros::group;
use reqwest::Client as Reqwest;
use tokio;
use serenity::http::Http;
use serenity::prelude::*;
use utilities::global_data::*;
use crate::handlers::event_handler::event_handler::Handler;
use tracing::error;

mod handlers;
mod commands;
mod utilities;

use crate::commands::math::*;
use crate::commands::utilities::*;
use crate::commands::owner::*;

#[group]
#[commands(multiply, ping, quit)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    // gets token, exits if no token
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable `RUST_LOG` to `debug`.
    tracing_subscriber::fmt::init();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILDS
        | GatewayIntents::AUTO_MODERATION_CONFIGURATION
        | GatewayIntents::AUTO_MODERATION_EXECUTION;

    let http = Http::new(&token);

    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
    .max_connections(5)
    .connect_with(
        sqlx::sqlite::SqliteConnectOptions::new()
            .filename("database.sqlite")
            .create_if_missing(true),
    )
    .await
    .expect("Couldn't connect to database");

    let connection = database.clone();

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");

    let handler = Handler {
        database,
        is_loop_running: AtomicBool::new(false),
    };

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(
        Configuration::new()
        .owners(owners)
        .dynamic_prefix(|ctx, msg| {
            Box::pin(async move {
                if msg.is_private() { // if private message, return default prefix
                    Some("-".to_string())
                } else {
                    // if guild message, return guild prefix
                    // implement guild settings hashmap and return prefix

                    let prefix = {
                        let data = ctx.data.read().await;
                        let guild_settings = data.get::<GuildSettingsContainer>().unwrap();
                        let pf = guild_settings.read().await;
                        let database = data.get::<DatabaseConnectionContainer>().unwrap().clone();

                        match pf.get(&msg.guild_id.unwrap().get()) {
                            Some(guild) => guild.prefix.clone(),
                            None => {
                                
                                // if no guild settings found, 
                                // create new database entry and return default prefix

                                let (guild_id, owner_id) = {
                                    let guild = msg.guild(&ctx.cache).unwrap();
                                
                                    (i64::from(guild.id), i64::from(guild.owner_id))
                                };

                                sqlx::query!(
                                    "INSERT INTO guild_settings (
                                        guild_id,
                                        prefix,
                                        owner_id
                                    ) VALUES (?, ?, ?)",
                                    guild_id,
                                    "-",
                                    owner_id
                                ).execute(&database).await.unwrap();

                                "-".to_string()
                            }
                        }
                    };

                    Some(prefix)
                }
            })
        })
        .prefix("")
        .on_mention(Some(bot_id))
    );

    let mut client =
        Client::builder(&token, intents)
        .framework(framework)
        .event_handler(handler).await.expect("Err creating client");

    let guild_settings = sqlx::query!("SELECT * FROM guild_settings")
        .fetch_all(&connection)
        .await
        .expect("Couldn't fetch guild settings");

    let mut guild_settings_map = HashMap::new();

    for guild_setting in guild_settings {
        let guild_id = guild_setting.guild_id as u64;
        let guild_settings = GuildSettings {
            prefix: guild_setting.prefix,
            owner_id: guild_setting.owner_id as u64,
            mute_type: guild_setting.mute_style,
            mute_role: guild_setting.mute_role_id.unwrap_or_default() as u64
        };

        guild_settings_map.insert(guild_id, guild_settings);
    }

    let reqwest_client = Reqwest::new();

    {
        let mut data = client.data.write().await;
        data.clear();
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<DatabaseConnectionContainer>(connection);
        data.insert::<GuildSettingsContainer>(Arc::new(RwLock::new(guild_settings_map)));
        data.insert::<ReqwestClientContainer>(Arc::new(reqwest_client));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
