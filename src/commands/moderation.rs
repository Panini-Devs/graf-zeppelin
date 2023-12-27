use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message, builder::{CreateEmbed, CreateMessage}
};
use crate::utilities::parsing;

#[command]
#[usage = "<@member> <reason>"]
#[description = "Bans the given member from the server."]
#[required_permissions("BAN_MEMBERS")]
#[only_in(guilds)]
#[min_args(1)]
/// Bans the given member from the server.
async fn ban(context: &Context, message: &Message, mut args: Args) -> CommandResult {

    let text = args.single::<String>().unwrap();

    let mut reason = args.remains().unwrap_or("No reason provided.").to_string();

    reason.push_str(format!(" | banned by {}", message.author.id).as_str());

    let parsed = parsing::parse_user(&text, context, message.guild_id.unwrap()).await;

    let member = match parsed {
        Ok(member) => member,
        Err(_) => {
            message.reply(&context.http, "Cannot find member.").await?;
            return Ok(());
        }
    };

    let dm_embed = CreateEmbed::new()
        .color(0x008b_0000)
        .description(format!("You have been banned from {} for `{}`", message.guild_id.unwrap().to_string(), reason));

    let create_message = CreateMessage::new()
        .embeds(vec![dm_embed])
        .content("The ban hammer has spoken.");

    let dm = member.user.dm(&context.http, create_message).await;

    match dm {
        Ok(_) => (),
        Err(_) => {
            message.channel_id.say(&context.http, "Failed to send DM user.").await?;
            //return Ok(());
        }
    }

    let res = member.ban_with_reason(&context.http, 7, reason).await;

    match res {
        Ok(_) => (),
        Err(_) => {
            message.reply(&context.http, "Failed to ban member. Give the bots its needed perms / roles, then try again.").await?;
            return Ok(());
        }
    }

    message.reply(&context.http, format!("Banned {}", member.user.tag())).await?;

    Ok(())
}

#[command]
#[usage = "<@member> <reason>"]
#[description = "Kicks the given member from the server."]
#[required_permissions("KICK_MEMBERS")]
#[only_in(guilds)]
#[min_args(1)]
/// Kicks the given member from the server.
async fn kick(context: &Context, message: &Message, mut args: Args) -> CommandResult {

    let text = args.single::<String>().unwrap();

    let mut reason = args.single::<String>().unwrap_or("No reason provided.".to_string());

    reason.push_str(format!(" | banned by {}", message.author.id).as_str());

    let parsed = parsing::parse_user(&text, context, message.guild_id.unwrap()).await;

    let member = match parsed {
        Ok(member) => member,
        Err(_) => {
            message.reply(&context.http, "Cannot find member.").await?;
            return Ok(());
        }
    };

    let dm_embed = CreateEmbed::new()
        .color(0x008b_0000)
        .description(format!("You have been banned from {} for `{}`", message.guild_id.unwrap().to_string(), reason));

    let create_message = CreateMessage::new()
        .embeds(vec![dm_embed])
        .content("The ban hammer has spoken.");

    let dm = member.user.dm(&context.http, create_message).await;

    match dm {
        Ok(_) => (),
        Err(_) => {
            message.channel_id.say(&context.http, "Failed to send DM user.").await?;
            //return Ok(());
        }
    }

    let res = member.kick_with_reason(&context.http, &reason).await;

    match res {
        Ok(_) => (),
        Err(_) => {
            message.reply(&context.http, "Failed to kick member. Give the bots its needed perms / roles, then try again.").await?;
            return Ok(());
        }
    }

    message.reply(&context.http, format!("Kicked {}", member.user.tag())).await?;

    Ok(())

}

