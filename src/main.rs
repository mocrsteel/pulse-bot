use poise::Event;
use serenity::prelude::*;

use pulse_bot::commands::channels;
use pulse_bot::globals::DISCORD_TOKEN;
use pulse_bot::structs::user;

#[tokio::main]
async fn main() {
    // Just some code to extract structs from Serenity to get an idea on content and structure.
    // async fn get_info() {
    //     let http = Http::new(&DISCORD_TOKEN);

    //     let guild = http
    //         .get_guild(*GUILD_ID)
    //         .await
    //         .expect("Erro getting guild info");
    //     let channels = http
    //         .get_channels(*GUILD_ID)
    //         .await
    //         .expect("error getting channel info");
    //     let events = http
    //         .get_scheduled_events(*GUILD_ID, true)
    //         .await
    //         .expect("error getting events");

    //     let mut file = File::create("./PartialGuild.json").expect("Error while creating file.");
    //     write!(file, "{}", to_string(&guild).unwrap()).unwrap();
    //     let mut file = File::create("./GuildChannels.json").expect("Error while creating file.");
    //     write!(file, "{}", to_string(&channels).unwrap()).unwrap();
    //     let mut file = File::create("./SchefuledEvents.json").expect("Error while creating file.");
    //     write!(file, "{}", to_string(&events).unwrap()).unwrap();
    // }
    // get_info().await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![channels::age(), channels::create_channel()],
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    if let Event::Ready { data_about_bot } = event {
                        println!("Bot is ready and connected to {}", data_about_bot.user.id);
                    } else {
                        println!("Got event in event handler: {:?}", event.name());
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(&*DISCORD_TOKEN)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(user::Data {})
            })
        });

    framework.run().await.unwrap();
}
