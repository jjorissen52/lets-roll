extern crate serenity;
use lets_roll::roll::{roll,cmd,Result};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot && msg.content.starts_with("/r") {
            let outbox: String;
            let result: Result = cmd(&msg.content);
            match result {
                Result::Invalid(explanation) => {
                    let default = format!("Your message \"{}\" did not contain a valid command.", msg.content);
                    outbox = format!("<@{}>: {}", msg.author.id, explanation.unwrap_or(default))
                }
                Result::Valid(commands, explanation, explain) => {
                    let dice = &commands[0];
                    let (roll_history, total) = roll(dice);
                    let show_roll: String;
                    if explain {
                        let explain_roll = explanation.unwrap_or(roll_history);
                        show_roll = format!("{} = {}", explain_roll, total);
                    } else {
                        show_roll = format!("{}", total);
                    }
                    outbox = format!("🎲 `{cmd}` <@{author}>  `{show_roll}` 🎲", author=msg.author.id, cmd=dice, show_roll=show_roll);
                }
                Result::TooBig(complaint) => {
                    outbox = format!("🎲 <@{}>: `{cmd}` {complaint} 🎲", msg.author.id, cmd=msg.content, complaint=complaint);
                }
            }
            let res = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(outbox);
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