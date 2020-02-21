use regex::Regex;
use rand::distributions::{Distribution, Uniform};

fn is_match(_match: regex::Match) -> bool {
    return _match.as_str().chars().count() == 0
}

lazy_static! {
    // user input to parse
    static ref DEFAULT: Regex = Regex::new(r"^/(r|rx|xr)\s*$").unwrap();
    static ref BASIC: Regex = Regex::new(r"^/(r|rx|xr)\s+(-?\d{1,4}d-?\d{1,10})$").unwrap();
    static ref BASIC_ADD: Regex = Regex::new(r"^/(r|rx|xr)\s+(-?\d{1,4}d-?\d{1,10})\s*(\+\s*\d{1,10}|-\s*\d{1,10})$").unwrap();
    static ref TOO_BIG: Regex = Regex::new(r"^/(r|rx|xr)\s+(-?\d{5,}d-?\d+|-?\d+d-?\d{10,})?").unwrap();
}

#[derive(PartialEq, Debug)]
pub enum Parsed {
    Invalid(Option<String>),
    Basic(Vec<Action>, Option<String>, bool), // bool is `explain`
    TooBig(String),
}

pub fn cmd(original: &String) -> Parsed {
    // takes a command such as 1d20 and returns vec!["1d20", ], is_valid, explain
    let result: Parsed;

    if DEFAULT.is_match(original) {
        let caps = DEFAULT.captures(original).unwrap();
        let explain = caps.get(1).map_or(false, |m| m.as_str().contains("x"));
        let dice = "1d20";
        let roll_action = str_to_roll(dice);
        let cmd_vec = vec![roll_action];
        result = Parsed::Basic(cmd_vec, Some(String::from("(The default roll is 1d20)")), explain);
    } else if BASIC.is_match(original) {
        let caps = BASIC.captures(original).unwrap();
        let explain = caps.get(1).map_or(false, |m| m.as_str().contains("x"));
        let dice = caps.get(2).map_or("", |m| m.as_str());
        let roll_action = str_to_roll(dice);
        let cmd_vec = vec![roll_action];
        result = Parsed::Basic(cmd_vec, None, explain);
    } else if BASIC_ADD.is_match(original) {
        let caps = BASIC_ADD.captures(original).unwrap();
        let explain = caps.get(1).map_or(false, |m| m.as_str().contains("x"));
        let dice = caps.get(2).map_or("", |m| m.as_str());
        let modifier = caps.get(3).map_or("+0", |m| m.as_str());
        let roll_action = str_to_roll(dice);
        let modify_action = str_to_modify(modifier);
        let cmd_vec = vec![roll_action, modify_action];
        result = Parsed::Basic(cmd_vec, None, explain)
    }  else if TOO_BIG.is_match(original) {
        result = Parsed::TooBig(String::from("Hey, that roll is too big..."))
    } else {
        result = Parsed::Invalid(Some(format!("Your command `{}` was not valid.", original)))
    }
    return result;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Action {
    Roll(bool, u32, u64), // is_negative, num_rolls, num_sides
    Modify(bool, u64), // is_negative, modifier
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Roll(is_negative, num_sides, num_faces) => {
                if *is_negative { format!("-{}d{}", num_sides, num_faces) } else { format!("{}d{}", num_sides, num_faces) }
            }
            Action::Modify(is_negative, value) => {
                if *is_negative { format!("-{}", value) } else { format!("{}", value) }
            }
        }
    }
}

pub fn perform_command(command: Parsed) -> (Option<String>, i128) {
    let mut explanation: String;
    let mut total:i128 = 0;
    match command {
        Parsed::Basic(actions, _explanation, explain) => {
            explanation = _explanation.unwrap_or(String::from(""));
            for action in actions {
                match action {
                    Action::Roll(is_negative, num_rolls, num_faces) => {
                        let (roll_history, result) = roll(Action::Roll(is_negative, num_rolls, num_faces));
                        let _result = result as i128;
                        total += _result;
                        if explain {
                            explanation += (if result < 0 {format!("- {}", roll_history)} else {format!("+ {}", roll_history)}).as_str();
                        }
                    }
                    Action::Modify(is_negative, value) => {
                        let _result = if is_negative {-1 * (value as i128)} else {value as i128};
                        total += _result;
                        if explain {
                            explanation += (if is_negative {format!("- {}", value)} else {format!("{}", value)}).as_str()
                        }
                    }
                }
            }
            if explain {
                return (Some(explanation), total);
            }
            return (None, total);
        }
        _ => {
            std::panic!("Command type not implemented!")
        }
    }
}


lazy_static! {
    // commands to parse and perform
    static ref ROLL: Regex = Regex::new(r"(-?)(\d+)d(-?)(\d+)").unwrap();
    static ref MODIFY: Regex = Regex::new(r"^(\+|-)\s*(\d+)$").unwrap();
}

pub fn str_to_roll(dice: &str) -> Action {
    let captures = ROLL.captures(dice).unwrap();
    let negative = captures.get(1).map_or(false, |m| is_match(m)) ^ captures.get(3).map_or(false, |m| is_match(m)); // XOR; only one negative
    let num_rolls = captures.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let num_sides = captures.get(4).map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    return Action::Roll(negative, num_rolls, num_sides)
}

pub fn str_to_modify(modifier: &str) -> Action {
    let captures = MODIFY.captures(modifier).unwrap();
    let minus = captures.get(1).map_or(false, |m| m.as_str() == "-");
    let value = captures.get(4).map_or(0, |m| m.as_str().parse::<u64>().unwrap());
    return Action::Modify(minus, value);
}

pub fn roll(dice: Action) -> (String, i64) {
    // takes a string such as 1d20, -1d20 and returns the result of a single roll
    match dice {
        Action::Roll(is_negative, num_rolls, num_sides) => {
            let mut rng = rand::thread_rng();

            // the else prevents an empty range eg 0..0 from causing a panic.
            let die = if num_sides > 1 {Uniform::from(0..num_sides)} else {Uniform::from(0..1)};

            let mut total: i64 = 0;
            let mut roll_history = String::new();
            for _roll in 0..num_rolls {
                // if num_sides == 0 or 1, give us num_sides, otherwise, do a roll
                let face = if num_sides > 1 {die.sample(&mut rng) + 1} else {num_sides} as i64;
                total += face;
                roll_history += &format!(" + {}", face).to_string()
            }
            if roll_history.chars().count() > 3 {
                roll_history = String::from(&roll_history[3..]);
            } else {
                roll_history = format!("{}", 0)
            }

            let result: String;
            if is_negative {
                total *= -1;
                result = format!("-({})", roll_history).to_string();
            } else {
                result = format!("({})", roll_history).to_string();
            }
            return (String::from(result), total)
        }
        _ => {
            std::panic!("roll only supports Action::Roll instances!")
        }
    }


}