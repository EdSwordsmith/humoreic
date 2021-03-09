#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;
use serenity::{
    framework::standard::StandardFramework,
    prelude::*,
};

pub mod commands;
pub mod database;
pub mod entities;
pub mod handler;
pub mod schema;

use commands::*;
use database::*;
use handler::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("☭"))
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
