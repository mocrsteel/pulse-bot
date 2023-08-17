use serenity::model::channel::ChannelType;

/// Channel types as per [serenity]. Refer to the [serenity] docs for more info:
/// [serenity::model::channel::ChannelType].
#[derive(Debug, poise::ChoiceParameter)]
pub enum ChannelCategoryOptions {
    /// Text channel.
    #[name = "Text"]
    Text,
    /// Private text channel.
    #[name = "Private"]
    Private,
    /// Voice channel
    #[name = "Voice"]
    Voice,
    /// Category (not voice, text or thread)
    #[name = "Category"]
    Category,
    #[name = "News"]
    News,
    #[name = "NewsThread"]
    NewsThread,
    #[name = "PrivateThread"]
    PrivateThread,
    #[name = "PublicThread"]
    PublicThread,
    #[name = "Stage"]
    Stage,
    #[name = "Directory"]
    Directory,
}

impl ChannelCategoryOptions {
    pub fn as_model(&self) -> ChannelType {
        match self {
            ChannelCategoryOptions::Text => ChannelType::Text,
            ChannelCategoryOptions::Private => ChannelType::Private,
            _ => todo!(),
        }
    }
}
