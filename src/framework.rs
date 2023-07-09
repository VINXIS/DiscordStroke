use std::collections::HashSet;
use serenity::{framework::StandardFramework, model::prelude::UserId};

use crate::commands::GENERAL_GROUP;

pub fn get_framework(owners: HashSet<UserId>, bot_id: UserId) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c
            .prefix("!")
            .on_mention(Some(bot_id))
            .delimiter("::")
            .no_dm_prefix(true)
            .owners(owners)
        )
        .group(&GENERAL_GROUP)
}