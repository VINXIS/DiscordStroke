use std::time::Duration;

use chrono::Utc;
use serenity::{client::Context, model::channel::Message, framework::standard::{macros::command, CommandResult}};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Utc::now();
    let mut message = msg.channel_id.say(&ctx.http, format!("Message timestamp: {}\nStart: {}\nSending: {}", msg.timestamp, start, Utc::now())).await?;
    let content = message.content.clone();
    message.edit(&ctx.http, |m| m.content(content.to_owned() + &format!("\nEnd: {}\nTime taken since start: {:?}", Utc::now(), (Utc::now() - start).to_std().unwrap_or(Duration::new(0, 0))))).await?;
    Ok(())
}
