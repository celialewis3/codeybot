use serenity::model::prelude::Reaction;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::id::*;
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
use tokio::net::TcpListener;
use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {


    /* TODO: This currently doesn't work as intended. */
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

        /* Codey will add a role to a member given the react used */

        let emoji = reaction.emoji.as_data();
        let gid = reaction.guild_id.unwrap();
        let user_id = reaction.user_id.unwrap();
        let member_result = gid.member(&ctx, user_id).await;
        let mut member = match member_result {
            Ok(v) => v,
            Err(e) => {println!("{}", e); return},
        };

        let role_to_add = match emoji.as_str() {
            "🍓" => RoleId(826230179097608202),
            "\u{1FAD0}" => RoleId(826230344243347466),
            "\u{1F347}" => RoleId(826245452805963820),
            _ => return,
         };

         if let Err(why) = member.add_role(&ctx, role_to_add).await {
            println!("Error assigning role: {}", why);
         }

     }

    /* Bot responds to messages */

    async fn message(&self, ctx: Context, msg: Message) {

        match msg.content.as_str() {
            "!surprise" => {
                let path = vec!["images/surprised.png"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },
        
            "!jam" => {
                let path = vec!["images/pingujam.gif"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },
        
            "!realizing" => {
                let path = vec!["images/realization.jpg"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },
        
            "!hearts" => {
                let path = vec!["images/hearts.jpg"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },
        
            "!cornjail" => {
                let path = vec!["images/cornjail.png"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },
        
            "!heh" => {
                let path = vec!["images/heh.png"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },
        
            "!panic" => {
                let path = vec!["images/panic.jpg"];
                if let Err(why) = msg.channel_id.send_files(&ctx.http, path, |m| m.content("") ).await {
                    println!("Error sending file: {}", why);
                }
            },

            "!test" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, 
                    "codeybot is working!").await {
                        println!("Error {}", why);
                    }
            },
            
        
            "!twitch" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, 
                    "You can find my twitch channel at https://www.twitch.tv/celiacode").await {
                        println!("Error {}", why);
                    }
            },
        
            _ => {},
        
        }

    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();


    // This binds to the env variable given by Heroku
    let port = env::var("PORT").unwrap_or_else(|_| "1234".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let mut listener = TcpListener::bind(addr).await.unwrap();


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