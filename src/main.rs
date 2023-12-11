use std::env;
use std::collections::HashSet;
use std::sync::Arc;
use serenity::framework::StandardFramework;
use serenity::framework::standard::Configuration;
use serenity::framework::standard::macros::group;
use serenity::gateway::ShardManager;
use tokio;
use serenity::http::Http;
use serenity::prelude::*;
use crate::handlers::event_handler::event_handler::Handler;
use tracing::error;

mod handlers;
mod commands;

use crate::commands::math::*;
use crate::commands::meta::*;
use crate::commands::owner::*;


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

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

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().owners(owners).prefix("-"));

    let mut client =
        Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler).await.expect("Err creating client");  
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
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
