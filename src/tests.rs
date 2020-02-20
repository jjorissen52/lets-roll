#[cfg(test)]
mod tests {
    use crate::roll::{cmd, roll};

    #[test]
    fn parse_basic_roll() {
        let commands = vec!["/r 1d20", "/r 2d6"];
        let rolls = vec!["1d20", "2d6"];
        for i in 0..commands.len() {
            let command = commands[i].to_string();
            let (cmd_vec, is_valid, explain) = cmd(&command);
            assert_eq!(cmd_vec[0], rolls[i]);
            assert!(is_valid);
            assert!(!explain);
        }
    }

    #[test]
    fn parse_explained_roll() {
        let commands = vec!["/rx 1d20", "/rx 2d6"];
        let rolls = vec!["1d20", "2d6"];
        for i in 0..commands.len() {
            let command = commands[i].to_string();
            let (cmd_vec, is_valid, explain) = cmd(&command);
            assert_eq!(cmd_vec[0], rolls[i]);
            assert!(is_valid);
            assert!(explain);
        }
    }

    #[test]
    fn reject_huge_rolls() {
        let commands = vec!["/rx 50000d20", "/rx 2d60000000000000"];
        for command in commands {
            let (_, is_valid, _) = cmd(&String::from(command));
            assert!(is_valid);
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