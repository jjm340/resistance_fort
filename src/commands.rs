use std::io;
use std::str::FromStr;

use crate::improvements::Improvement;

#[derive(Debug)]
pub enum Command<'a> {
    Quit,
    PurchaseMenu,
    DoPurchase(&'a Improvement),
    StealFood,
    StealCurrency,
}

impl<'a> FromStr for Command<'a> {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Command<'a>, Self::Err> {
        match input.trim() {
            "0" | "q" | "quit" => Ok(Command::Quit),
            "1" | "p" | "purchase" => Ok(Command::PurchaseMenu),
            "2" | "sf" | "steal food" => Ok(Command::StealFood),
            "3" | "sc" | "steal currency" => Ok(Command::StealCurrency),
            cmd_text => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown command \"{cmd_text}\""),
            )),
        }
    }
}
