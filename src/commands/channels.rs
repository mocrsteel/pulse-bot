

use serde_json::json;
use serenity::http::Http;
use serenity::json::hashmap_to_json_map;
use serenity::model::channel::ChannelType;
use serenity::model::user::User;
use std::collections::HashMap;

use crate::autocomplete::database::autocomplete_name;
use crate::commands::choices;
use crate::globals::Context;
use crate::globals::{Error, DISCORD_TOKEN, GUILD_ID};

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Command to create a channel
#[poise::command(slash_command)]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "Name"]
    #[autocomplete = "autocomplete_name"]
    name: String,
    #[description = "Description"] description: String,
    #[description = "parent"] parent: choices::ChannelCategoryOptions,
) -> Result<(), Error> {
    let http = Http::new(&DISCORD_TOKEN);
    let map = HashMap::from([
        ("name".to_string(), json!(name.clone())),
        ("type".to_string(), json!(ChannelType::Text.num())),
        ("topic".to_string(), json!(description.clone())),
    ]);
    dbg!(parent.as_model());
    dbg!(hashmap_to_json_map(map.clone()));
    http.create_channel(*GUILD_ID, &hashmap_to_json_map(map), None)
        .await?;

    ctx.say(format!("Created channel {}: ```{}```", name, description))
        .await?;

    Ok(())
}

///Testing some documentatino things
pub fn empty() {}
