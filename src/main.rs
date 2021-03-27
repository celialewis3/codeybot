use serenity::model::prelude::Reaction;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::id::*;
use serenity::model::*;
use serenity::model::prelude::Member;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use dotenv::dotenv;
use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn guild_member_addition(&self, ctx: Context, gid: GuildId, person: Member)
    {
        let mut greeting: String = "welcome to codeyclub, ".to_owned();
        let name: &str = person.user.name.as_str();

        greeting.push_str(name);

        if let Err(why) = ChannelId(824783680354385920).say(&ctx.http, greeting).await {
            println!("Error sending greeting: {}", why);
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let saying = reaction.emoji.as_data();
        if let Err(why) = reaction.channel_id.say(&ctx.http, saying).await {
            println!("Error sending message: {}", why);
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "!surprise" {
            let path = vec!["images/surprised.png"];
            if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                println!("Error sending file: {}", why);
            }
        }

        if msg.content == "!jam" {
            let path = vec!["images/pingujam.gif"];
            if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                println!("Error sending file: {}", why);
            }
        }

        if msg.content == "!hearts" {
            let path = vec!["images/hearts.jpg"];
            if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                println!("Error sending file: {}", why);
            }
        }

        if msg.content == "!cornjail" {
            let path = vec!["images/cornjail.png"];
            if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                println!("Error sending file: {}", why);
            }
        }


        if msg.content == "!heh" {
            let path = vec!["images/heh.png"];
            if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                println!("Error sending file: {}", why);
            }
        }


        if msg.content == "!panic" {
            let path = vec!["images/panic.jpg"];
            if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                println!("Error sending file: {}", why);
            }
        }

        if msg.content == "!twitch" {
            if let Err(why) = msg.channel_id.say(&ctx.http, 
                "You can find my twitch channel at https://www.twitch.tv/celiacode").await {
                    println!("Error {}", why);
                }
        }
    }

    
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");
    
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::builder(&token)
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