#[macro_use]
extern crate cute;
extern crate serenity;
use lets_roll::roll::{cmd, Parsed, perform_command};

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
            let command: Parsed = cmd(&msg.content);
            match command {
                Parsed::Invalid(explanation) => {
                    let default = format!("Your message \"{}\" did not contain a valid command.", msg.content);
                    outbox = format!("<@{}>: {}", msg.author.id, explanation.unwrap_or(default))
                }
                Parsed::Basic(commands, explanation, explain) => {
                    let action_vec = commands.clone();
                    let (explanation, roll_result) = perform_command(Parsed::Basic(commands, explanation, explain));
                    let show_roll: String;
                    if explain {
                        show_roll = format!("{} = {}", explanation.unwrap(), roll_result);
                    } else {
                        show_roll = format!("{}", roll_result);
                    }
                    let actions = c![action.to_string(), for action in action_vec];
                    outbox = format!("🎲 `{actions}` <@{author}>  `{show_roll}` 🎲", author=msg.author.id, actions=actions.join(" + "), show_roll=show_roll);
                }
                Parsed::TooBig(complaint) => {
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