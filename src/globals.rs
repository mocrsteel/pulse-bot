use std::env;

use once_cell::sync::Lazy;

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
