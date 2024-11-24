use std::str::FromStr;

use crate::{
    commands::Command, improvements::IMPROVEMENTS, input::fetch_input, utils::log_and_exit,
};

pub fn main_menu<'a>() -> Option<Command<'a>> {
    let menu_string = format!(
        r#"
    Please select an option: 
    1) Purchase improvement
    2) Steal food
    3) Steal currency
"#
    );

    print!("{menu_string}");
    let input = fetch_input();

    let parsed = Command::from_str(&input);

    match parsed {
        Ok(Command::PurchaseMenu) => show_purchase_menu(),
        Ok(Command::StealCurrency) => Some(Command::StealCurrency),
        Ok(Command::StealFood) => Some(Command::StealFood),
        Ok(Command::Quit) => log_and_exit(None),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
        Ok(anything) => {
            println!("{:?} command not valid here", anything);
            None
        }
    }
}

pub fn show_purchase_menu<'a>() -> Option<Command<'a>> {
    println!("What would you like to purchase?");
    let mut improvements_display = String::new();

    for (_, improvement) in IMPROVEMENTS.iter() {
        let formatted = format!("{}: {}\n", improvement.name, improvement.description);
        improvements_display += &formatted[..];
    }

    println!("{improvements_display}");

    loop {
        let input = fetch_input();

        if input == "quit" {
            println!("Okay, to previous menu");
            return None;
        }

        match IMPROVEMENTS.get(&input) {
            Some(selected) => {
                println!("{} selected!", selected.name);
                Some(Command::DoPurchase(selected));
            }
            None => {
                println!("Input not recognized, try again");
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_examples_returns_results() {
        // let mut improvements_hash = HashMap::new();
        // improvements_hash.insert("Tavern", 12345);

        // let result = improvements_hash.get("tavern");

        // assert_eq!(result, Some(&12345));
    }
}
