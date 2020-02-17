#[macro_use] extern crate lazy_static;
extern crate serenity;
extern crate regex;
extern crate rand;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use regex::Regex;
use std::{env};
use rand::distributions::{Distribution, Uniform};

fn is_match(_match: regex::Match) -> bool {
    return _match.as_str().chars().count() == 0
}

fn roll(dice: &String) -> (String, i64) {
    // takes a string such as 1d20, -1d20 and returns the result of a single roll
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?)(\d+)d(-?)(\d+)").unwrap();
    }

    let captures = RE.captures(dice).unwrap();
    let negative = captures.get(1).map_or(false, |m| is_match(m)) ^ captures.get(3).map_or(false, |m| is_match(m)); // XOR; only one negative
    let num_rolls = captures.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    let num_sides = captures.get(4).map_or(0, |m| m.as_str().parse::<i64>().unwrap());

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..num_sides);

    let mut total = 0;
    let mut roll_history = String::new();
    for _roll in 0..num_rolls {
        let face = die.sample(&mut rng);
        total += face;
        roll_history += &format!(" + {}", face).to_string()
    }
    roll_history = String::from(&roll_history[3..]);

    let mut result = String::new();
    if negative {
        total *= -1;
        result += format!("-({}) = {}", roll_history, total).as_str();
    } else {
        result += format!("({}) = {}", roll_history, total).as_str();
    }
    return (String::from(result), total)
}

fn cmd(original: &String) -> (Vec<&str>, bool, bool) {
    // takes a command such as /r 1d20 and returns vec!["1d20", ]
    lazy_static! {
        static ref RE: Regex = Regex::new(r"/(r|rh|hr)\s*(-?\d+d-?\d+)").unwrap();
    }
    let mut cmd_vec= Vec::new();
    let mut provide_history = false;

    if RE.is_match(original) {
        let caps = RE.captures(original).unwrap();
        let dice = caps.get(2).map_or("INVALID", |m| m.as_str());
        provide_history = caps.get(1).map_or(false, |m| m.as_str().contains("h"));
        cmd_vec.push(dice);
        return (cmd_vec, true, provide_history);
    }
    return (cmd_vec, false, provide_history)

}

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