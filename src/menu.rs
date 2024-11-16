use crate::Command;
use std::error::Error;
use std::io::{self};
use std::str::FromStr;

pub fn show_menu(cmd: Command) {
    match cmd {
        Command::Main => main_menu(),
    }
}

pub fn main_menu() -> Option<Command> {
    let mut input = String::new();

    let menu_string = format!(
        r#"C
    Please select an option: 
    1) Purchase improvement
    2) Steal food
    3) Steal currency
"#
    );

    print!("{menu_string}");

    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error occured reading input: {}", e);
        std::process::exit(1);
    } else {
        let parsed = Some(Command::from_str(&input));

        match parsed {
            Ok(cmd) => Ok(cmd),
            Err(not_recognized) => Err(Box::new(not_recognized)),
        }
    }
}
