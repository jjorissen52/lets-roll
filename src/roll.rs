use regex::Regex;
use rand::distributions::{Distribution, Uniform};

fn is_match(_match: regex::Match) -> bool {
    return _match.as_str().chars().count() == 0
}

pub fn roll(dice: &String) -> (String, i64) {
    // takes a string such as 1d20, -1d20 and returns the result of a single roll
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?)(\d+)d(-?)(\d+)").unwrap();
    }

    let captures = RE.captures(dice).unwrap();
    let negative = captures.get(1).map_or(false, |m| is_match(m)) ^ captures.get(3).map_or(false, |m| is_match(m)); // XOR; only one negative
    let num_rolls = captures.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
    let num_sides = captures.get(4).map_or(0, |m| m.as_str().parse::<i64>().unwrap());

    let mut rng = rand::thread_rng();

    // the else prevents an empty range eg 0..0 from causing a panic.
    let die = if num_sides > 1 {Uniform::from(0..num_sides)} else {Uniform::from(0..1)};

    let mut total = 0;
    let mut roll_history = String::new();
    for _roll in 0..num_rolls {
        // if num_sides == 0 or 1, give us num_sides, otherwise, do a roll
        let face = if num_sides > 1 {die.sample(&mut rng) + 1} else {num_sides};
        total += face;
        roll_history += &format!(" + {}", face).to_string()
    }
    if roll_history.chars().count() > 3 {
        roll_history = String::from(&roll_history[3..]);
    } else {
        roll_history = format!("{}", 0)
    }

    let result: String;
    if negative {
        total *= -1;
        result = format!("-({})", roll_history).to_string();
    } else {
        result = format!("({})", roll_history).to_string();
    }
    return (String::from(result), total)
}

#[derive(PartialEq)]
pub enum Result {
    Invalid(Option<String>),
    Valid(Vec<String>, Option<String>, bool), // bool is `explain`
    TooBig(String),
}

pub fn cmd(original: &String) -> Result {
    // takes a command such as 1d20 and returns vec!["1d20", ], is_valid, explain
    lazy_static! {
        static ref DEFAULT: Regex = Regex::new(r"^/(r|rx|xr)\s*$").unwrap();
        static ref ACCEPT: Regex = Regex::new(r"^/(r|rx|xr)\s+(-?\d{1,4}d-?\d{1,10})?$").unwrap();
        static ref TOO_BIG: Regex = Regex::new(r"^/(r|rx|xr)\s+(-?\d{5,}d-?\d+|-?\d+d-?\d{10,})?$").unwrap();
    }
    let result: Result;

    if DEFAULT.is_match(original) {
        let caps = DEFAULT.captures(original).unwrap();
        let explain = caps.get(1).map_or(false, |m| m.as_str().contains("x"));
        let dice = "1d20";
        let cmd_vec = vec![String::from(dice)];
        result = Result::Valid(cmd_vec, Some(String::from("(The default roll is 1d20)")), explain);
    } else if ACCEPT.is_match(original) {
        let caps = ACCEPT.captures(original).unwrap();
        let explain = caps.get(1).map_or(false, |m| m.as_str().contains("x"));
        let dice = caps.get(2).map_or("", |m| m.as_str());
        let cmd_vec = vec![String::from(dice)];
        result = Result::Valid(cmd_vec, None, explain);
    } else if TOO_BIG.is_match(original) {
        result = Result::TooBig(String::from("Hey, that roll is too big..."))
    } else {
        result = Result::Invalid(Some(String::from("Your roll did not match a valid command.")))
    }

    return result;
}