use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, id}
};

#[command("ban")]
#[usage = "<member>"]
#[description = "Bans the given member from the server."]
#[required_permissions(BAN_MEMBERS)]
#[only_in(guilds)]
#[min_args(1)]
/// Bans the given member from the server.
async fn ban(context: &Context, message: &Message, mut args: Args) -> CommandResult {

    let member = {

        let mention = match message.mentions.get(0) {
            Some(mention) => mention,
            None => {
                let user_id = match args.single::<id::UserId>() {
                    Ok(user_id) => {
                        let guild = message.guild(&context.cache).unwrap().clone();
                        let user = guild.member(&context.http, user_id).await.unwrap().user;
                        user
                    },
                    Err(_) => {
                        message.reply(context, "Provide a valid user to ban!").await?;
                        return Ok(());
                    }
                };
            }
        };
    };
    



    Ok(())
}