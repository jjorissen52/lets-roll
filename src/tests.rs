#[cfg(test)]
mod tests {
    use crate::roll::{cmd, roll, str_to_roll, str_to_modify, Parsed, Action};

    #[test]
    fn parse_basic_roll() {
        let commands = vec!["/r 1d20", "/r 2d6", "/r -1d6", "/r -1d6"];
        let rolls = vec!["1d20", "2d6", "-1d6", "-1d6", "-1d6"];
        for i in 0..commands.len() {
            let command = commands[i].to_string();
            let result = cmd(&command);
            match result {
                Parsed::Basic(cmd_vec, _, should_explain) => {
                    assert_eq!(cmd_vec[0].to_string(), rolls[i]);
                    let roll_as_action = str_to_roll(rolls[i]);
                    assert_eq!(cmd_vec[0], roll_as_action);
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
                Parsed::Basic(cmd_vec, _, should_explain) => {
                    assert_eq!(cmd_vec[0].to_string(), rolls[i]);
                    let roll_as_action = str_to_roll(rolls[i]);
                    assert_eq!(cmd_vec[0], roll_as_action);
                    assert!(should_explain)
                }
                _ => { std::panic!("These were valid rolls!") }
            }
        }
    }

    // #[test]
    // fn parse_multi_part_roll() {
    //     let commands = vec!["/r 1d20", "/r 2d6", "/r 1d6 + 8", "/r 1d6 - 12"];
    //     let rolls = vec![vec!["1d20"], vec!["2d6"], vec!["1d6", "+8"], vec!["1d6", "-12"]];
    //     for i in 0..commands.len() {
    //         let command = commands[i].to_string();
    //         let parsed_command = cmd(&command);
    //         match parsed_command {
    //             Parsed::Basic(cmd_vec, _, should_explain) => {
    //                 for j in 0..cmd_vec.len() {
    //                     let roll_as_action = str_to_roll(rolls[i][j]);
    //                     assert_eq!(cmd_vec[j], roll_as_action);
    //                 }
    //                 assert!(!should_explain)
    //             }
    //             _ => { std::panic!("These were valid rolls!") }
    //         }
    //     }
    // }

    #[test]
    fn reject_huge_rolls() {
        let commands = vec!["/rx 50000d20", "/rx 2d60000000000000"];
        for command in commands {
            let result = cmd(&String::from(command));
            match result {
                Parsed::TooBig(_) => {}
                _ => { std::panic!("These were rolls were much too large!") }
            }
        }
    }

    #[test]
    fn rolls_succeed() {
        let rolls = vec!["1d20", "2d6", "-1d-5", "0d-5", "5d0", "100d1000000000000"];
        for r in rolls {
            let action = str_to_roll(r);
            roll(action);
        }
    }
}