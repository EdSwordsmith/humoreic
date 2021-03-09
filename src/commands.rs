use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use crate::entities::admins::*;
use crate::entities::bans::*;
use crate::entities::guilds::*;
use crate::entities::messages::*;
use crate::handler::DBConnection;
use serenity::model::id::ChannelId;

#[group]
#[commands(ping, setup, admin, addadmin, ban, unban, del)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Não quero saber! 0.75 para ti!").await?;
    msg.reply(
        ctx,
        "https://scontent.flis6-1.fna.fbcdn.net/v/t1.0-9/70961516_2959".to_owned()
            + "708937378994_4689648200359870464_o.jpg?_nc_cat=111&ccb=1-3&_nc_sid"
            + "=825194&_nc_ohc=D4Gg4D4CrvcAX-9A812&_nc_ht=scontent.flis6-1.fna&oh="
            + "af2a5651c2bc4c2e55affee070113262&oe=6068A3E0",
    )
    .await?;
    Ok(())
}

#[command]
async fn setup(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();

    if is_admin(&conn, *msg.author.id.as_u64() as i64) {
        if let Some(guild_id) = msg.guild_id {
            create_guild(&conn, guild_id.0 as i64, msg.channel_id.0 as i64);
        }
    }

    Ok(())
}

#[command]
async fn admin(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();

    if is_admin(&conn, msg.author.id.0 as i64) {
        msg.reply(&ctx, "Ya bro, you an admin!").await?;
    } else {
        msg.reply(&ctx, "Fuck off, bro!").await?;
    }

    Ok(())
}

#[command]
async fn addadmin(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();

    if is_admin(&conn, msg.author.id.0 as i64) {
        for mention in msg.mentions.iter() {
            create_admin(&conn, mention.id.0 as i64);
        }
    }

    Ok(())
}

#[command]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();

    if is_admin(&conn, msg.author.id.0 as i64) {
        for mention in msg.mentions.iter() {
            create_ban(&conn, mention.id.0 as i64);
        }
    }

    Ok(())
}

#[command]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();

    if is_admin(&conn, msg.author.id.0 as i64) {
        for mention in msg.mentions.iter() {
            rm_ban(&conn, mention.id.0 as i64);
        }
    }

    Ok(())
}

#[command]
async fn del(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<DBConnection>().unwrap();
    let conn = pool.get().unwrap();
    let args: Vec<&str> = msg.content.split(" ").collect();

    if is_admin(&conn, msg.author.id.0 as i64) {
        if args.len() >= 2 {
            let message_id = args.get(1).unwrap().parse::<i64>()?;
            let message = find_message(&conn, message_id, msg.guild_id.unwrap().0 as i64);
            let guilds = get_guilds(&conn);
            let embed_ids = message.embed_ids.as_object().unwrap();
            let msg_ids = message.msg_ids.as_object().unwrap();

            for g in guilds {
                let channel = ChannelId(g.channel_id as u64);
                channel
                    .delete_message(
                        &ctx.http,
                        embed_ids.get(&g.id.to_string()).unwrap().as_u64().unwrap(),
                    )
                    .await
                    .expect("APAGA MALUCO!");
                for v in msg_ids.get(&g.id.to_string()).unwrap().as_array().unwrap() {
                    channel
                        .delete_message(&ctx.http, v.as_u64().unwrap())
                        .await
                        .expect("APAGA MALUCO!");
                }
            }
            delete_message(&conn, message.id);
        }
    }

    Ok(())
}
