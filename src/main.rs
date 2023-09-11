use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

mod datastorage;
use datastorage::save_channelid;

#[group]
#[commands(ping)]
#[commands(count)]
#[commands(setchannel)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    datastorage::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" has 0 messages in this server (I haven't started counting them yet, numb nuts)")
        .build();
    msg.reply(ctx, response).await?;

    Ok(())
}

#[command]
async fn setchannel(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: only allow setchannel to work if user has manage server permissions
    
    // Store the channel id in the file.
    let channel = msg.channel_id.to_string();
    save_channelid(channel);

    // Get the channel for the mention response.
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(ch) => ch,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return Ok(());
        },
    };

    let response = MessageBuilder::new()
        .push("Concierge has set up his desk in ")
        .mention(&channel)
        .build();

    msg.reply(ctx, response).await?;
    Ok(())
}