#[cfg(test)]
mod tests {
    use crate::roll::{cmd, roll, Result};

    #[test]
    fn parse_basic_roll() {
        let commands = vec!["/r 1d20", "/r 2d6"];
        let rolls = vec!["1d20", "2d6"];
        for i in 0..commands.len() {
            let command = commands[i].to_string();
            let result = cmd(&command);
            match result {
                Result::Valid(cmd_vec, _, should_explain) => {
                    assert_eq!(cmd_vec[0], String::from(rolls[i]));
                    assert!(!should_explain)
                }
                _ => { std::panic!("These were valid rolls!") }
            }
        }
    }

    #[test]
    fn parse_explained_roll() {
        let commands = vec!["/rx 1d20", "/rx 2d6"];
        let rolls = vec!["1d20", "2d6"];
        for i in 0..commands.len() {
            let command = commands[i].to_string();
            let result = cmd(&command);
            match result {
                Result::Valid(cmd_vec, _, should_explain) => {
                    assert_eq!(cmd_vec[0], String::from(rolls[i]));
                    assert!(should_explain)
                }
                _ => { std::panic!("These were valid rolls!") }
            }
        }
    }

    #[test]
    fn reject_huge_rolls() {
        let commands = vec!["/rx 50000d20", "/rx 2d60000000000000"];
        for command in commands {
            let result = cmd(&String::from(command));
            match result {
                Result::TooBig(complaint) => {}
                _ => { std::panic!("These were rolls were much too large!") }
            }
        }
    }

    #[test]
    fn rolls_succeed() {
        let rolls = vec!["1d20", "2d6", "-1d-5", "0d-5", "5d0", "100d1000000000000"];
        for r in rolls {
            roll(&String::from(r));
        }
    }
}