use std::env;

use serenity::{
    framework::{
        StandardFramework
    }, 
    prelude::{
        GatewayIntents
    }, 
    Client
};


#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let discord_key = match env::var("KEY") {
        Ok(key) => key,
        Err(_) => panic!("KEY not found in .env"),
    };

    let intents = GatewayIntents::all();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"));

    let mut client = match Client::builder(&discord_key, intents)
        .framework(framework)
        .await {
        Ok(client) => client,
        Err(err) => panic!("Failed to create client: {:?}", err)
    };

    match client.start().await {
        Ok(()) => (),
        Err(err) => panic!("Client failed to log in: {:?}", err)
    }

    println!("Client has logged in");
}