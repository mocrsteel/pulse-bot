use serde::{Serialize, Deserialize};
use serenity::model::channel::ChannelType;

#[derive(Debug, Serialize)]
pub struct CreateChannel {
    pub name: String,
    pub channel_type: ChannelType,
    pub topic: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub name: String,
    pub id: i64,
    pub channel_type: ChannelType,
    pub position: i64,
}
