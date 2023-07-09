mod ping;

use serenity::{framework::standard::macros::group};
use ping::PING_COMMAND;

#[group]
#[commands(PING)]
pub struct General;