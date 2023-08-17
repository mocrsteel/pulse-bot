use std::env;

use once_cell::sync::Lazy;

use crate::structs::user::Data;

pub static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("DISCORD_TOKEN")
        .expect("missing DISCORD_TOKEN in environment")
});

pub static GUILD_ID: Lazy<u64> = Lazy::new(|| {
    env::var("GUILD_ID")
        .expect("missing GUILD_ID in the environment")
        .parse::<u64>()
        .expect("GUILD_ID is not an unsigned integer")
});

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
