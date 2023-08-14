use serde_json::json;
use serenity::http::Http;
use serenity::json::{hashmap_to_json_map, Value};
use serenity::model::channel::{
    Channel, ChannelCategory, ChannelType, GuildChannel, PartialGuildChannel,
};
use serenity::model::user::User;
use std::collections::HashMap;
use std::convert;

use crate::globals::{DISCORD_TOKEN, GUILD_ID};
use crate::structs::{ChannelInfo, CreateChannel, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
///
/// # To do
///
/// * Allow selection of Category (Text, Category, Voice, Stage, ...)
/// * Figure out how to give options in Discord slash command arguments.
/// * Figure out how to filter options given for 'parent' as I only want to show categories and not regular channels.
///
#[poise::command(slash_command)]
pub async fn create_channel(
    ctx: Context<'_>,
    #[description = "Name"] name: String,
    #[description = "Description"] description: String,
    #[description = "parent"] parent: Option<Channel>,
    #[description = "kind"] kind: Option<ChannelCategory>,
) -> Result<(), Error> {
    let http = Http::new(&DISCORD_TOKEN);
    let mut map = HashMap::from([
        ("name".to_string(), json!(name.clone())),
        ("type".to_string(), json!(ChannelType::Text.num())),
        ("topic".to_string(), json!(description.clone())),
    ]);
    if let Some(parent) = parent {
        map.insert("parent_id".to_string(), json!(parent.id()));
    } else {
    }
    dbg!(hashmap_to_json_map(map.clone()));
    http.create_channel(*GUILD_ID, &hashmap_to_json_map(map), None)
        .await?;

    ctx.say(format!("Created channel {}: ```{}```", name, description))
        .await?;

    Ok(())
}
