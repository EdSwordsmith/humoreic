use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping, setup)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("☭"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Não quero saber! 0.75 para ti!\n".to_owned() +
        "https://scontent.flis6-1.fna.fbcdn.net/v/t1.0-9/70961516_2959" +
        "708937378994_4689648200359870464_o.jpg?_nc_cat=111&ccb=1-3&_nc_sid" +
        "=825194&_nc_ohc=D4Gg4D4CrvcAX-9A812&_nc_ht=scontent.flis6-1.fna&oh=" +
        "af2a5651c2bc4c2e55affee070113262&oe=6068A3E0").await?;
    Ok(())
}

#[command]
async fn setup(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Ainda não funfa").await?;
    Ok(())
}
