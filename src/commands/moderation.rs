use std::time::{SystemTime, UNIX_EPOCH};

use crate::utilities::{global_data::GuildSettingsContainer, parsing};
use serenity::{
    builder::{CreateEmbed, CreateMessage, EditChannel},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[usage = "<@member> <reason>"]
#[description = "Bans the given member from the server."]
#[required_permissions(BAN_MEMBERS)]
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

    let dm_embed = CreateEmbed::new().color(0x008b_0000).description(format!(
        "You have been banned from {} for `{}`",
        message.guild_id.unwrap(),
        reason
    ));

    let create_message = CreateMessage::new()
        .embeds(vec![dm_embed])
        .content("The ban hammer has spoken.");

    let dm = member.user.dm(&context.http, create_message).await;

    match dm {
        Ok(_) => (),
        Err(_) => {
            message
                .channel_id
                .say(&context.http, "Failed to send DM user.")
                .await?;
            //return Ok(());
        }
    }

    let res = member.ban_with_reason(&context.http, 7, reason).await;

    match res {
        Ok(_) => (),
        Err(_) => {
            message
                .reply(
                    &context.http,
                    "Failed to ban member. Give the bots its needed perms / roles, then try again.",
                )
                .await?;
            return Ok(());
        }
    }

    message
        .reply(&context.http, format!("Banned {}", member.user.tag()))
        .await?;

    Ok(())
}

#[command]
#[usage = "<@member> <reason>"]
#[description = "Kicks the given member from the server."]
#[required_permissions(KICK_MEMBERS)]
#[only_in(guilds)]
#[min_args(1)]
/// Kicks the given member from the server.
async fn kick(context: &Context, message: &Message, mut args: Args) -> CommandResult {
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

    let dm_embed = CreateEmbed::new().color(0x008b_0000).description(format!(
        "You have been banned from {} for `{}`",
        message.guild_id.unwrap(),
        reason
    ));

    let create_message = CreateMessage::new()
        .embeds(vec![dm_embed])
        .content("The ban hammer has spoken.");

    let dm = member.user.dm(&context.http, create_message).await;

    match dm {
        Ok(_) => (),
        Err(_) => {
            message
                .channel_id
                .say(&context.http, "Failed to send DM user.")
                .await?;
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

    message
        .reply(&context.http, format!("Kicked {}", member.user.tag()))
        .await?;

    Ok(())
}

#[command]
#[usage = "<@member> <reason>"]
#[description = "Mutes the given member for a given / default duration."]
#[aliases("timeout", "mute")]
#[required_permissions(MODERATE_MEMBERS)]
#[only_in(guilds)]
#[min_args(1)]
/// Mutes the given member for a given / default duration.
async fn mute(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let text = args.single::<String>().unwrap();

    let mut time = args.single::<u128>().unwrap_or(0);

    if time < 1 {
        let data = context.data.read().await;

        let settings = data.get::<GuildSettingsContainer>().unwrap();

        let pf = settings.read().await;

        time = pf
            .get(&message.guild_id.unwrap().get())
            .unwrap()
            .default_mute_duration as u128;
    }

    time *= 1000;

    if time > 2419000000 {
        time = 2419000000;
    }

    let unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get unix epoch time")
        .as_millis();

    time += unix_epoch;

    let timestamp = serenity::model::Timestamp::from_millis(time as i64).unwrap();

    let mut reason = args.remains().unwrap_or("No reason provided.").to_string();

    reason.push_str(format!(" | muted by {}", message.author.id).as_str());

    let parsed = parsing::parse_user(&text, context, message.guild_id.unwrap()).await;

    let member = match parsed {
        Ok(member) => member,
        Err(_) => {
            message.reply(&context.http, "Cannot find member.").await?;
            return Ok(());
        }
    };

    let dm_embed = CreateEmbed::new().color(0x008b_0000).description(format!(
        "You have been muted in {} for `{}`",
        message.guild_id.unwrap(),
        reason
    ));

    let create_message = CreateMessage::new()
        .embeds(vec![dm_embed])
        .content("Timeout");

    let dm = member.user.dm(&context.http, create_message).await;

    match dm {
        Ok(_) => (),
        Err(_) => {
            message
                .channel_id
                .say(&context.http, "Failed to send DM user.")
                .await?;
            //return Ok(());
        }
    }

    let res = member
        .clone()
        .disable_communication_until_datetime(&context.http, timestamp)
        .await;

    match res {
        Ok(_) => (),
        Err(_) => {
            message.reply(&context.http, "Failed to mute member. Give the bots its needed perms / roles, then try again.").await?;
            return Ok(());
        }
    }

    message
        .reply(
            &context.http,
            format!("Muted {} for {}", member.user.tag(), reason),
        )
        .await?;

    Ok(())
}

#[command]
#[usage = "<@member> <reason>"]
#[description = "Unmutes the given member."]
#[aliases("untimeout", "unmute")]
#[required_permissions(MODERATE_MEMBERS)]
#[only_in(guilds)]
#[min_args(1)]
/// Unmutes the given member.
async fn unmute(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let text = args.single::<String>().unwrap();

    let parsed = parsing::parse_user(&text, context, message.guild_id.unwrap()).await;

    let member = match parsed {
        Ok(member) => member,
        Err(_) => {
            message.reply(&context.http, "Cannot find member.").await?;
            return Ok(());
        }
    };

    let res = member.clone().enable_communication(&context.http).await;

    match res {
        Ok(_) => (),
        Err(_) => {
            message.reply(&context.http, "Failed to unmute member. Give the bots its needed perms / roles, then try again.").await?;
            return Ok(());
        }
    }

    message
        .reply(&context.http, format!("Unmuted {}", member.user.tag()))
        .await?;

    Ok(())
}

#[command]
#[description = "Creates slowmode in the given channel."]
#[usage = "<seconds>"]
#[aliases("slowmode", "sm")]
#[required_permissions(MANAGE_CHANNELS)]
#[only_in(guilds)]
#[min_args(1)]
/// Creates slowmode in the given channel.
async fn slowmode(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let seconds = args.single::<u16>().unwrap();

    let channel = message.channel_id;

    let edit = EditChannel::new().rate_limit_per_user(seconds);

    let res = channel.edit(&context.http, edit).await;

    match res {
        Ok(_) => (),
        Err(_) => {
            message.reply(&context.http, "Failed to set slowmode. Give the bots its needed perms / roles, then try again.").await?;
            return Ok(());
        }
    }

    message
        .reply(
            &context.http,
            format!("Set slowmode to {} seconds", seconds),
        )
        .await?;

    Ok(())
}
