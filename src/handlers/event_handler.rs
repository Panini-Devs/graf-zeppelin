
pub mod event_handler {
    use serenity::async_trait;
    use serenity::client::EventHandler;
    use serenity::model::channel::Message;
    use serenity::model::gateway::Ready;
    use serenity::all::{Context, ResumedEvent};
    use tracing::info;
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
        async fn message(&self, ctx: Context, msg: Message) {
            // TODO: add advanced command handler + database connection
        }

        // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
        // a READY payload is sent by Discord. This payload contains data like the current user's guild
        // Ids, current user data, private channels, and more.
        //
        // In this case, just print what the current user's username is.
        async fn ready(&self, _: Context, ready: Ready) {
            println!("{} is connected!", ready.user.name);
        }

        async fn resume(&self, _: Context, _: ResumedEvent) {
            info!("Resumed");
        }
    }
}