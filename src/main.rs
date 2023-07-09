mod commands;
mod framework;
mod handler;
mod utils;

use std::{env, error::Error, collections::HashSet};
use serenity::{prelude::{GatewayIntents}, Client, http::Http};
use crate::{handler::Handler, framework::get_framework};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv()?; // Initialize to fetch .env file

    let discord_key = env::var("KEY").expect("KEY not found in .env");
    let http = Http::new(&discord_key);
    let intents = GatewayIntents::all();
    let event_handler = Handler;

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(err) => panic!("Could not access application info: {:?}", err)
    };

    let mut client = match Client::builder(&discord_key, intents)
        .framework(get_framework(owners, bot_id))
        .event_handler(event_handler)
        .await {
        Ok(client) => client,
        Err(err) => panic!("Failed to create client: {:?}", err)
    };

    match client.start().await {
        Ok(()) => (),
        Err(err) => panic!("Client failed to log in: {:?}", err)
    }

    println!("Client has logged in");
    Ok(())
}