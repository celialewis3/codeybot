use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::*;
use serenity::model::prelude::Member;
use serenity::{async_trait, framework::standard::Args};
use serenity::{model::prelude::Reaction, prelude::TypeMapKey};

use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;

use reqwest;
use tokio_postgres::{Error, NoTls};

mod ghibli;

#[group]
#[commands(ghibli)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /* TODO: This currently doesn't work as intended. */
    async fn guild_member_addition(&self, ctx: Context, _gid: GuildId, person: Member) {
        let mut greeting: String = "welcome to codeyclub, ".to_owned();
        let name: &str = person.user.name.as_str();

        greeting.push_str(name);

        if let Err(why) = ChannelId(824783680354385920).say(&ctx.http, greeting).await {
            println!("Error sending greeting: {}", why);
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        // If the reaction is toggled in the #getting-started channel, remove roll
        if reaction.channel_id == ChannelId(826244579497213998) {
            let emoji = reaction.emoji.as_data();
            let gid = reaction.guild_id.unwrap();
            let user_id = reaction.user_id.unwrap();
            let member_result = gid.member(&ctx, user_id).await;
            let mut member = match member_result {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let role_to_remove = match emoji.as_str() {
                "🍓" => RoleId(826230179097608202),
                "\u{1FAD0}" => RoleId(826230344243347466),
                "\u{1F347}" => RoleId(826245452805963820),
                _ => return,
            };

            if let Err(why) = member.remove_role(&ctx, role_to_remove).await {
                println!("Error assigning role: {}", why);
            }
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
            Err(e) => {
                println!("{}", e);
                return;
            }
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
        /* Universally, we want to increment points for talking */

        let db_client = ctx.data.read().await;
        let new_client = db_client.get::<DbClient>().unwrap();

        let user_id = msg.author.id.as_u64().to_string();
        let member_name = msg.author.name.clone();

        increment_points(&new_client, user_id.clone(), member_name.clone()).await;
        make_hungrier(&new_client).await;

        match msg.content.as_str() {
            // Once a day, you can get a gift on the server.
            // The gift is randomized.
            // Given to the person who called the command.
            "!catch" => {
                let mut congrats_message: String = "congrats! you caught a pokemon, ".to_owned();

                congrats_message.push_str(member_name.as_str());

                let path = vec!["images/gengar.jpg"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(congrats_message))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!hunger" => {
                let statement = "select hunger from codey  
                where name = 'Codey'";

                let rows = new_client.query(statement, &[]).await;

                let member_row = rows.unwrap();

                let member_row2 = member_row.get(0).unwrap();

                println!("{:?}", member_row2);

                let hunger: i32 = member_row2.get(0);

                let message = format!("Codey's hunger points are {} out of 100", hunger);

                if let Err(why) = msg.channel_id.say(&ctx, message).await {
                    println!("Error sending file: {}", why);
                }
            }

            "!feed" => {
                let decrease_hunger = "update codey 
                                set hunger = 100;";

                if let Err(e) = new_client.execute(decrease_hunger, &[]).await {
                    println!("Error occured while feeding: {}", e);
                }
                let path = vec!["images/eating.gif"];

                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| {
                        m.content("Codey happily eats the grapes you offer him!")
                    })
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!skitty" => {
                let path = vec!["images/skitty.gif"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!points" => {
                let db_client = ctx.data.read().await;
                let new_client = db_client.get::<DbClient>().unwrap();

                let statement = "select points from members  
                                where userid = $1";

                let id = msg.author.id.as_u64().to_string();

                let rows = new_client.query(statement, &[&id]).await;

                let member_row = rows.unwrap();

                let member_row2 = member_row.get(0).unwrap();

                let points: i32 = member_row2.get(0);

                print!("{}", points);

                let mut message = "you have ".to_owned();
                // message.push_str(points.to_string());

                message.push_str(points.to_string().as_str());
                message.push_str(" codey points!");

                match msg.reply(&ctx, message).await {
                    Ok(_e) => {}
                    Err(e) => {
                        println!("Error occured on message send in !points: {}", e);
                    }
                }
            }

            "!vip" => {
                // Check how many points you have

                let db_client = ctx.data.read().await;
                let new_client = db_client.get::<DbClient>().unwrap();

                let statement = "select points from members  
                                where userid = $1";

                let id = msg.author.id.as_u64().to_string();

                let rows = new_client.query(statement, &[&id]).await;

                let member_row = rows.unwrap();

                let member_row2 = member_row.get(0).unwrap();

                let points: i32 = member_row2.get(0);

                if points >= 50 {
                    match msg.reply(&ctx, "You are now a VIP!").await {
                        Ok(_e) => {}
                        Err(e) => {
                            println!("Error occured on message send in !vip: {}", e);
                        }
                    }

                    let vip_role = RoleId(827983030957113390);

                    let gid = msg.guild_id.unwrap();
                    let user_id = msg.author.id;
                    let member_result = gid.member(&ctx, user_id).await;
                    let mut member = match member_result {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };

                    match member.add_role(&ctx, vip_role).await {
                        Ok(_e) => {}
                        Err(e) => {
                            println!("Error occured on add role in !vip: {}", e);
                        }
                    }
                } else {
                    let remainder = 50 - points;

                    let message = format!("You need {} more points to become a VIP!", remainder);
                    match msg.reply(&ctx, message).await {
                        Ok(_e) => {}
                        Err(e) => {
                            println!("Error occured on message reply in !vip: {}", e);
                        }
                    }
                }

                // If you have 50 points, assign you the VIP role and give you SPECIAL PRIVILEGES SUCH AS
                // access to grape juice, a Very Cool Lounge, and More!!!!!
            }

            "!8ball" => {
                // let mut rng = rng

                // let response = match rng.gen_range(0..6) {
                //     0 => "i am not in the mood to answer your question lolz",
                //     1 => "Ayyyyyy lmao... no",
                //     2 => "-__- listen..... i cant tell you, but..... Maybe? Lol",
                //     3 => "are you kidding? LMAOOOO",
                //     4 => "girl XDDDDD",
                //     _ => "Bro for sure",
                // };

                //msg.reply(&ctx, response).await;
            }

            "!cry" => {
                let path = vec!["images/crying.jpg"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!rocket" => {
                let path = vec!["images/teamrocket.png"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| {
                        m.content("prepare for trouble?? make it double!!")
                    })
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!surprise" => {
                let path = vec!["images/surprised.png"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!jam" => {
                let path = vec!["images/pingujam.gif"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!realizing" => {
                let path = vec!["images/realization.jpg"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!hearts" => {
                let path = vec!["images/hearts.jpg"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!cornjail" => {
                let path = vec!["images/cornjail.png"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!heh" => {
                let path = vec!["images/heh.png"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!panic" => {
                let path = vec!["images/panic.jpg"];
                if let Err(why) = msg
                    .channel_id
                    .send_files(&ctx.http, path, |m| m.content(""))
                    .await
                {
                    println!("Error sending file: {}", why);
                }
            }

            "!test" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "codeybot is working!").await {
                    println!("Error {}", why);
                }
            }

            "!twitch" => {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "You can find my twitch channel at https://www.twitch.tv/celiacode",
                    )
                    .await
                {
                    println!("Error {}", why);
                }
            }

            _ => {}
        }
    }
}

async fn increment_points(new_client: &tokio_postgres::Client, id: String, member_name: String) {
    /* First, check if the member exists in the DB */
    let query = "select 1
                    from members
                    where userid = $1";

    let q_rows = new_client.execute(query, &[&id]).await;

    let result = (&q_rows.unwrap()).to_owned();

    /* If result is 0, they are not in the DB. Add them in using INSERT INTO */
    if result == 0 {
        let add_member = "INSERT INTO members (userid, points, name)
        VALUES ($1, 0, $2);";

        if let Err(e) = new_client.execute(add_member, &[&id, &member_name]).await {
            println!("Error occcured while adding new member: {}", e);
        }
    }

    /* Now increment the points the member has */
    let statement = "update members 
    set points = points + 1 
    where userid = $1";

    if let Err(e) = new_client.execute(statement, &[&id]).await {
        println!("Error occurred while increasing points: {}", e);
    }
}

async fn make_hungrier(client: &tokio_postgres::Client) {
    let decrement_hunger = "update codey 
                                set hunger = hunger - 1 
                                ";

    if let Err(e) = client.execute(decrement_hunger, &[]).await {
        println!("Error: {}", e);
    }
}

struct DbClient;

impl TypeMapKey for DbClient {
    type Value = tokio_postgres::Client;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let (db_client, connection) =
        tokio_postgres::connect(env::var("DB_STRING").unwrap().as_str(), NoTls).await?;

    /* The connection object performs the actual communication with the database,
    so spawn it off to run on its own. */

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Binds the bot to a port. This is required by Discord for the bot to run.
    bind().await;

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    /* Login with a Discord bot token from the environment and also include the DB connection as part of the context;
    this is done by inserting it into the type map. */
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<DbClient>(db_client)
        .await
        .expect("Error creating client");

    /* start listening for events by starting a single shard */
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}

async fn bind() {
    let _listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();
}

// This command is created using the #[command] macro.
#[command]
async fn ghibli(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let movie_name = args.rest();

    let response = reqwest::get("https://ghibliapi.herokuapp.com/films").await?;

    let films = response.json::<Vec<ghibli::Film>>().await?;

    let movie_info = ghibli::movie_info(movie_name.to_string(), &films);

    msg.reply(ctx, movie_info).await?;

    Ok(())
}
