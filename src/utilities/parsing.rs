use serenity::{all::User, client::Context, Error};


pub async fn parse_user(text: &str, context: &Context) -> Result<User, Error> {

    let to_trim: &[_] = &['<', '@', '!', '>'];
    
    let stripped = text.trim_matches(to_trim);

    let id = stripped.parse().unwrap();

    let user = context.http.get_user(id).await;

    user
}