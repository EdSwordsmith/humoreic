use humoreic::{create_guild, get_guild, get_guilds};
use humoreic::PgPool;
use humoreic::establish_connection;
use dotenv::dotenv;
use std::env;
use regex::Regex;

use serenity::{async_trait, model::{channel::Message, gateway::Ready, id::ChannelId}, prelude::*};

use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

struct DBConnection;
impl TypeMapKey for DBConnection {
    type Value = PgPool;
}

#[group]
#[commands(ping, setup)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        let pool = data.get::<DBConnection>().unwrap();
        let conn = pool.get().unwrap();

        let guild_id = *msg.guild_id.unwrap().as_u64() as i64;
        let guild_data = get_guild(&conn, guild_id);
        let channel_id = *msg.channel_id.as_u64() as i64;

        let guild = msg.guild(&ctx.cache).await.unwrap();
        let guild_icon = guild.icon_url().unwrap();

        if !msg.author.bot && guild_data.channel_id == channel_id {
            let guilds = get_guilds(&conn);

            for g in guilds {
                let image_regex = Regex::new(r"(http(s?)://)([/|.|\w|\s|-])*\.(?:jpg|gif|png)").unwrap();

                let channel = ChannelId(g.channel_id as u64);
                match channel.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        if image_regex.is_match(&msg.content) {
                            e.image(&msg.content);
                        } else {
                            e.description(&msg.content);
                        }

                        e.author(|a| {
                            a.name(&msg.author.name);
                            a.icon_url(&msg.author.face());

                            a
                        });

                        e.footer(|f| {
                            f.text(&guild.name);
                            f.icon_url(&guild_icon);

                            f
                        });
                        
                        e
                    });
                    
                    m
                }).await {
                    Err(_) => println!("wtf are u doing"),
                    _ => ()
                };

                for attachment in msg.attachments.clone() {
                    if channel_id != *channel.as_u64() as i64 {
                        match channel.say(&ctx.http, attachment.url).await {
                            Err(_) => println!("brah learn to write Rust"),
                            _ => ()
                        };
                    }
                }
            }

            // Apagar mensagem depois de enviar a todos os servers
            // Caso não tenha enviado imagens/videos
            if msg.attachments.len() == 0 {
                match msg.delete(&ctx.http).await {
                    Err(_) => println!("wtf bro"),
                    _ => ()
                };
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    {
        let mut data = client.data.write().await;
        data.insert::<DBConnection>(establish_connection());
    }

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
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();

    if let Some(guild_id) = msg.guild_id {
        create_guild(&conn, *guild_id.as_u64() as i64, *msg.channel_id.as_u64() as i64);
    }

    Ok(())
}
