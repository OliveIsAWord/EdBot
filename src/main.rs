use std::fs;

mod command_types;
mod random;

use command_types::MessageResponse;

mod emoji_replace;
use emoji_replace::EmojiReplace;

use serenity::{
    async_trait, client,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

struct EdBotHandler {
    id: UserId,
    uwu: EmojiReplace,
}

impl EdBotHandler {
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            uwu: EmojiReplace::default(),
        }
    }
}

#[async_trait]
impl EventHandler for EdBotHandler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        // EdBot should ignore their own messages, or else they will get trapped in loops.
        if msg.author.id == self.id {
            return;
        }
        //println!("{:?}", msg);
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        if let Some(reply) = self.uwu.message_response(&msg).await {
            if let Err(why) = msg.channel_id.say(&ctx.http, &reply).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = fs::read_to_string("secret/token.txt")
        .expect("Error reading token file.")
        .trim()
        .to_owned();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let bot_id = client::parse_token(&token)
        .expect("Invalid token.")
        .bot_user_id;
    let edbot = EdBotHandler::new(bot_id);
    let mut client = Client::builder(&token)
        .event_handler(edbot)
        .await
        .expect("Error creating client.");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// use discord::model::Event;
// use discord::Discord;
// //use std::env;
// use std::fs;
//
// const TOKEN_PATH: &'static str = "secret/token.txt";
//
// fn main() {
// 	// Log in to Discord using a bot token from the environment.
// 	let discord = {
//         let token = fs::read_to_string(TOKEN_PATH).expect("Could not read bot token from file.");
//         Discord::from_bot_token(&token).expect("Login failed.")
//     };
// 	// Establish and use a websocket connection.
// 	let (mut connection, _) = discord.connect().expect("Connection failed.");
// 	println!("Ready.");
// 	loop {
// 		match connection.recv_event() {
// 			Ok(Event::MessageCreate(message)) => {
// 				println!("{} says: {}", message.author.name, message.content);
// 				if message.content == "!test" {
// 					let _ = discord.send_message(
// 						message.channel_id,
// 						"This is a reply (to the test.",
// 						"",
// 						false,
// 					);
// 				} else if message.content == "!quit" {
// 					println!("Quitting.");
// 					break;
// 				}
// 			}
// 			Ok(_) => {}
// 			Err(discord::Error::Closed(code, body)) => {
// 				println!("Gateway closed on us with code {:?}: {}.", code, body);
// 				break;
// 			}
// 			Err(err) => println!("Receive error: {:?}.", err),
// 		}
// 	}
// }
