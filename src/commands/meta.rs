use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    //msg.channel_id.say(&ctx.http, "Pong!").await?;

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms = since_the_epoch.as_millis();

    let latency = in_ms - msg.timestamp.timestamp_millis() as u128;

    let embed = CreateEmbed::new()
        .title("Pong!")
        .field("Latency", format!("Ping: {latency}ms"), false)

        ;
        //.footer()

    let builder = CreateMessage::new()
    .content("Hello, World!")
    .embed(embed);

    let msg = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = msg {
        println!("Error sending message: {why:?}");
    }

    Ok(())
}