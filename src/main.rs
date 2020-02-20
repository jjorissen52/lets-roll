extern crate serenity;
use lets_roll::roll::{roll,cmd};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot && msg.content.starts_with("/r") {
            let mut result = String::new();
            let (commands, valid, provide_history) = cmd(&msg.content);
            if !valid {
                result += format!("<@{}>: Your message \"{msg}\" did not contain a valid command. Be better.",
                                  msg.author.id, msg=msg.content).as_str();
            } else {
                let dice = String::from(commands[0]);
                let (roll_history, total) = roll(&dice);
                if provide_history {
                    result += format!("🎲 {cmd} <@{}>  {roll_history} 🎲", msg.author.id, cmd=commands[0], roll_history=roll_history).as_str();
                } else {
                    result += format!("🎲 {cmd} <@{}>  {total} 🎲", msg.author.id, cmd=commands[0], total=total).as_str();
                }
            }
            let res = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(result);
                m
            });
            let _ = msg.delete(&ctx);

            if let Err(why) = res {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("LETS_ROLL_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}