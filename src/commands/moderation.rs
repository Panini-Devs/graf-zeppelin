use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message
};
use crate::utilities::parsing;

#[command("ban")]
#[usage = "<member>"]
#[description = "Bans the given member from the server."]
#[required_permissions(BAN_MEMBERS)]
#[only_in(guilds)]
#[min_args(1)]
/// Bans the given member from the server.
async fn ban(context: &Context, message: &Message, mut args: Args) -> CommandResult {

    let text = args.single::<String>().unwrap();

    let reason = args.single::<String>().unwrap_or("No reason provided.".to_string());

    let parsed = parsing::parse_user(&text, context).await;

    let user = match parsed {
        Ok(user) => user,
        Err(_) => {
            message.reply(&context.http, "Cannot find user.").await?;
            return Ok(());
        }
    };

    let member = match context.http.get_member(message.guild_id.unwrap(), user.id).await {
        Ok(member) => member,
        Err(_) => {
            message.reply(&context.http, "Cannot find member in guild cache.").await?;
            return Ok(());
        }
    };

    let _ = member.ban_with_reason(&context.http, 7, reason).await;

    message.reply(&context.http, format!("Banned {}", user.tag())).await?;

    Ok(())
}