use std::env;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

mod datastorage;

#[group]
#[commands(ping,count,setchannel,initgather)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    datastorage::init();
    //logger::init();
    println!("Initializing framework");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    println!("Initializing bot");
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Starting client...");
    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    reply(msg, ctx, "Pong!".to_string()).await;
    Ok(())
}

#[command]
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .mention(&msg.author)
        .push(" has 0 messages in this server (I haven't started counting them yet, numb nuts)")
        .build();
    
    reply(msg, ctx, response).await; 

    Ok(())
}

#[command]
async fn initgather(ctx: &Context, msg: &Message) -> CommandResult {
    let mtx = tokio::sync::Mutex::new(0);
    let _guard = mtx.lock().await;
    reply(msg, ctx, "Beginning to gather messages...".to_owned()).await;
    let reply_msg = datastorage::start_gather_data(ctx, msg).await;
    reply(msg, ctx, reply_msg).await;
    Ok(())
}

#[command]
async fn setchannel(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: only allow setchannel to work if user has manage server permissions
    // Store the channel id in the file.
    let channel = msg.channel_id.to_string();
    let guild = msg.guild_id;
    let guild = guild.unwrap().0;
    datastorage::save_channelid(guild, channel);

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
    
    reply(msg, ctx, response).await;
    
    Ok(())
}

async fn does_msg_match_channel(msg: &Message, ctx: &Context) -> bool {
    // Get the channel for the mention response.
    let channel =msg.channel_id.to_channel(&ctx).await;

    let channel  = match channel {
        Ok(ch) => ch,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return false;
        },
    };

    let stored_id = datastorage::get_channelid(msg.guild_id.unwrap().0.to_string());
    let actual_id = channel.id().to_owned();

    match stored_id {
        Ok(id) => return id == actual_id.to_string(),
        Err(e) => {
            println!("Error retrieving stored id; no match ({})", e);
            return false
        }
    }
}

// Custom reply message only if we are in the correct channel.
async fn reply(msg: &Message, ctx: &Context, resp: String) {
    println!("beginning reply...");
    let res: Result<Message, SerenityError>;
    if does_msg_match_channel(msg, ctx).await {
        res = msg.reply(ctx,resp).await;
    } else {
        res = msg.reply(ctx, MessageBuilder::new()
            .push("This isn't the lobby sir; if I'm wrong, you should use the ")
            .push_mono("setchannel")
            .push(" command to move my lobby to this channel.")
            .build()
        ).await;
    }
    match res {
        Ok(_) => { 
            println!("{} {}", "Reply successful to", &msg.author.name);
        },
        Err(err) => { println!("Reply unsuccessful: {}", err.to_string()); }
    }
}