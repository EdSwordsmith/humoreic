use regex::Regex;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage},
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, Ready},
        id::ChannelId,
    },
    prelude::*,
};
use std::collections::HashMap;

use crate::database::get_db_connection;
use crate::database::PgPool;
use crate::entities::admins::*;
use crate::entities::bans::*;
use crate::entities::guilds::*;
use crate::entities::messages::*;
use crate::entities::reactions::*;

pub struct DBConnection;
impl TypeMapKey for DBConnection {
    type Value = PgPool;
}

pub struct Handler;

fn create_embed(
    m: &mut CreateMessage,
    msg: &Message,
    guild: &serenity::model::guild::Guild,
    guild_icon: &String,
) {
    let image_regex = Regex::new(r"^((http(s?)://)([^@|/|.|\w|\s|-])*\.(?:jpg|gif|png))$").unwrap();

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
}

async fn update_embeds(
    ctx: &Context,
    message: &SavedMessage,
    guilds: &Vec<Guild>,
    reactions: &HashMap<String, Vec<SavedReaction>>,
) {
    let embeds = message.embed_ids.as_object().unwrap();

    for g in guilds.iter() {
        let message_id = embeds.get(&g.id.to_string()).unwrap().as_u64().unwrap();
        let channel = ChannelId(g.channel_id as u64);
        let mut msg = channel
            .message(&ctx.http, message_id)
            .await
            .expect(&format!(
                "Couldn't send message to channel {} with guild {}",
                channel.0, g.id
            ));
        let mut fake_embed = msg.embeds.remove(0);
        fake_embed.fields = vec![];

        let mut embed = CreateEmbed::from(fake_embed);
        let mut text = String::new();

        let mut sorted: Vec<_> = reactions.iter().map(|(k, v)| (k, v.len())).collect();
        sorted.sort_by_key(|a| a.1);
        sorted.reverse();
        let mut i = 0;

        for (re, v) in sorted.iter() {
            if i % 10 == 0 {
                text += "\n";
            }

            text += &format!(" {} {} ", v, re);
            i += 1;
        }

        if reactions.len() > 0 {
            embed.field("Reactions", text, true);
        }

        channel
            .edit_message(&ctx.http, message_id, |edit| {
                edit.embed(|e| {
                    *e = embed;
                    e
                })
            })
            .await
            .expect(&format!(
                "Couldn't edit message to channel {} with guild {}",
                channel.0, g.id
            ));
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let conn = get_db_connection(&ctx).await;

        let guild_id = msg.guild_id.unwrap().0 as i64;
        let guild_data = get_guild(&conn, guild_id);
        let channel_id = msg.channel_id.0 as i64;

        let banned = is_banned(&conn, msg.author.id.0 as i64);

        if !msg.author.bot && guild_data.channel_id == channel_id {
            if banned {
                msg.delete(&ctx.http)
                    .await
                    .expect(&format!("Couldn't delete message {}", msg.id.0));
                return;
            }

            let guilds = get_guilds(&conn);
            let mut embed_ids = HashMap::new();
            let mut msg_ids = HashMap::new();

            for g in guilds {
                let tenor_regex =
                    Regex::new(r"^(http(s?)://)((tenor\.com[^@]*)|(media\.giphy\.com[^@]*)|(gph\.is[^@]*))$")
                        .unwrap();
                let video_regex =
                    Regex::new(r"^((http(s?)://)([^@|/|.|\w|\s|-])*\.(?:mp4|webm))$").unwrap();
                msg_ids.insert(g.id, vec![]);

                let channel = ChannelId(g.channel_id as u64);
                let guild = msg.guild(&ctx.cache).await.unwrap();
                let guild_icon = guild.icon_url().unwrap();
                match channel
                    .send_message(&ctx.http, |m| {
                        create_embed(m, &msg, &guild, &guild_icon);
                        m
                    })
                    .await
                {
                    Err(_) => eprintln!(
                        "Couldn't send message to channel {} with guild {}",
                        channel.0, guild.id.0
                    ),
                    Ok(message) => {
                        embed_ids.insert(g.id, message.id.0 as i64);
                    }
                };

                if tenor_regex.is_match(&msg.content) || video_regex.is_match(&msg.content) {
                    match channel.say(&ctx.http, &msg.content).await {
                        Err(_) => eprintln!(
                            "Couldn't send message to channel {} with guild {}",
                            channel.0, guild.id.0
                        ),
                        Ok(message) => {
                            msg_ids.get_mut(&g.id).unwrap().push(message.id.0 as i64);
                        }
                    };
                }

                for attachment in msg.attachments.clone() {
                    if channel_id != channel.0 as i64 {
                        match channel.say(&ctx.http, attachment.url).await {
                            Err(_) => eprintln!(
                                "Couldn't send message to channel {} with guild {}",
                                channel.0, guild.id.0
                            ),
                            Ok(message) => {
                                msg_ids.get_mut(&g.id).unwrap().push(message.id.0 as i64);
                            }
                        };
                    }
                }
            }

            // Apagar mensagem depois de enviar a todos os servers
            // Caso não tenha enviado imagens/videos
            if msg.attachments.len() == 0 {
                msg.delete(&ctx.http)
                    .await
                    .expect(&format!("Couldn't delete message {}", msg.id.0));
            } else {
                msg_ids.get_mut(&guild_id).unwrap().push(msg.id.0 as i64);
            }

            create_message(&conn, embed_ids, msg_ids);
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let conn = get_db_connection(&ctx).await;

        let message = find_message(
            &conn,
            reaction.message_id.0 as i64,
            reaction.guild_id.unwrap().0 as i64,
        );
        let guilds = get_guilds(&conn);
        let r = reaction.emoji.to_string();
        let user_id = reaction.user_id.unwrap().0 as i64;

        if is_admin(&conn, user_id) && r == "❌" {
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
                    .expect(&format!(
                        "Couldn't delete message {}",
                        reaction.message_id.0
                    ));
                for v in msg_ids.get(&g.id.to_string()).unwrap().as_array().unwrap() {
                    channel
                        .delete_message(&ctx.http, v.as_u64().unwrap())
                        .await
                        .expect(&format!(
                            "Couldn't delete message {}",
                            reaction.message_id.0
                        ));
                }
            }
            delete_message(&conn, message.id);
            return;
        }

        let mut reactions = get_reactions(&conn, message.id);
        if has_reaction(&reactions, &r, user_id) {
            reaction.delete(&ctx.http).await.expect(&format!(
                "Couldn't delete reaction in message {}",
                reaction.message_id.0
            ));
            return;
        }

        create_reaction(&conn, message.id, &r, user_id, reaction.channel_id.0 as i64);
        let dummy_reaction = SavedReaction {
            id: 0,
            reaction: String::new(),
            message_id: 0,
            user_id: 0,
            channel_id: 0,
        };
        if let Some(react) = reactions.get_mut(&r) {
            react.push(dummy_reaction);
        } else {
            reactions.insert(r.clone(), vec![dummy_reaction]);
        }

        update_embeds(&ctx, &message, &guilds, &reactions).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let conn = get_db_connection(&ctx).await;

        let message = find_message(
            &conn,
            reaction.message_id.0 as i64,
            reaction.guild_id.unwrap().0 as i64,
        );
        let guilds = get_guilds(&conn);
        let r = reaction.emoji.to_string();
        let user_id = reaction.user_id.unwrap().0 as i64;

        if is_admin(&conn, user_id) && r == "❌" {
            return;
        }

        let reactions = get_reactions(&conn, message.id);

        if !reaction_actually_exists(&reactions, &r, user_id, reaction.channel_id.0 as i64) {
            return;
        }

        delete_reaction(&conn, message.id, &r, user_id);
        let reactions = get_reactions(&conn, message.id);
        update_embeds(&ctx, &message, &guilds, &reactions).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::playing("memes for all EIC kind!"))
            .await;
    }
}
