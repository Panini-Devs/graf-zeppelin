pub mod event_handler {
    use serenity::async_trait;
    use serenity::client::EventHandler;
    use serenity::gateway::ActivityData;
    use serenity::model::channel::Message;
    use serenity::model::gateway::Ready;
    use serenity::all::{Context, ResumedEvent, Guild, OnlineStatus, UnavailableGuild};
    use tracing::info;

    use crate::utilities::global_data::{DatabaseConnectionContainer, GuildSettingsContainer, GuildSettings};
    pub struct Handler {
        pub database: sqlx::SqlitePool,
    }

    #[async_trait]
    impl EventHandler for Handler {
        // Set a handler for the `message` event - so that whenever a new message is received - the
        // closure (or function) passed will be called.
        //
        // Event handlers are dispatched through a threadpool, and so multiple events can be dispatched
        // simultaneously.
        async fn message(&self, _ctx: Context, _msg: Message) {
            // TODO: add advanced command handler + database connection
        }

        async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
            // write into database and hashmap
            if is_new == Some(true) {
                info!("Joined guild: {}", guild.name);
                info!("Guild ID: {}", guild.id);
                info!("Guild Owner ID: {}", guild.owner_id);
                info!("Guild Members: {}", guild.member_count);

                let data = ctx.data.read().await;
                let database = data.get::<DatabaseConnectionContainer>().unwrap().clone();
                let (guild_id, owner_id) = {
                    let guild_id = i64::from(guild.id);
                    let owner_id = i64::from(guild.owner_id);

                    (guild_id, owner_id)
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

                let owner_id_u64 = owner_id as u64;
                let guild_id_u64 = guild_id as u64;

                let data_to_set = GuildSettings {
                    prefix: "-".to_string(),
                    owner_id: owner_id_u64,
                    mute_type: "timeout".to_string(),
                    mute_role: 0
                };

                {
                    let mut guild_settings = data.get::<GuildSettingsContainer>().unwrap().write().await;
                    guild_settings.insert(guild_id_u64, data_to_set);
                }

                info!("Guild settings set complete for guild {}", guild.name);
            } else {
                info!("Connected to guild: {}", guild.name);
                info!("Guild ID: {}", guild.id);
                info!("Guild Owner ID: {}", guild.owner_id);
                info!("Guild Members: {}", guild.member_count);
            }
        }

        async fn guild_delete(&self, ctx: Context, _: UnavailableGuild, g: Option<Guild>) {
            let guild = g.unwrap();
            info!("Left guild: {}", guild.name);
            // write into database and hashmap
            {
                let data = ctx.data.read().await;
                let database = data.get::<DatabaseConnectionContainer>().unwrap().clone();
                let guild_id = i64::from(guild.id);
                sqlx::query!(
                    "DELETE FROM guild_settings WHERE guild_id = ?",
                    guild_id
                ).execute(&database).await.unwrap();
            }
        }

        // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
        // a READY payload is sent by Discord. This payload contains data like the current user's guild
        // Ids, current user data, private channels, and more.
        //
        // In this case, just print what the current user's username is.
        async fn ready(&self, context: Context, ready: Ready) {
            let http = &context.http;

            let api_version = ready.version;
            let bot_gateway = http.get_bot_gateway().await.unwrap();
            let bot_owner = http.get_current_application_info().await.unwrap().owner.expect("Couldn't get bot owner");
            let t_sessions = bot_gateway.session_start_limit.total;
            let r_sessions = bot_gateway.session_start_limit.remaining;
            let shard_info = ready.shard.unwrap();

            info!("Successfully logged into Discord as the following user:");
            info!("Bot username: {}", ready.user.tag());
            info!("Bot user ID: {}", ready.user.id);
            info!("Bot owner: {}", bot_owner.tag());

            let guild_count = ready.guilds.len();

            info!("Connected to shard {} out of a total of {} shards.", shard_info.id, shard_info.total);
            info!("Connected to the Discord API (version {api_version}) with {r_sessions}/{t_sessions} sessions remaining.");
            info!("Connected to and serving a total of {guild_count} guild(s).");

            let presence = format!("on {guild_count} guilds | -help");
            context.set_presence(Some(ActivityData::playing(presence)), OnlineStatus::Online);
        }

        async fn resume(&self, _: Context, _: ResumedEvent) {
            info!("Resumed!");
        }
    }
}