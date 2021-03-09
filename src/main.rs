#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use serenity::{framework::standard::StandardFramework, prelude::*};
use std::env;

mod commands;
mod database;
mod entities;
mod handler;
mod schema;

use commands::*;
use database::*;
use handler::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("pah! "))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

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
