pub mod character_record;
pub mod commands;
pub mod improvements;
pub mod input;
pub mod menu;
pub mod utils;

use character_record::ResourceType;
use commands::Command;
use menu::main_menu;

use crate::character_record::Character;
use core::panic;
use std::{cell::RefCell, fmt};

#[derive(Debug, Clone)]
pub struct CommonError {
    message: String,
}

impl CommonError {
    pub fn new(message: String) -> CommonError {
        CommonError { message }
    }
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommonError: {}", self.message)
    }
}

pub fn process_input<'a>() -> Command<'a> {
    match main_menu() {
        Some(cmd) => cmd,
        _ => {
            panic!("The main menu produced no valid command");
        }
    }
}

pub fn update<'a>(character: &RefCell<Character<'a>>, next_command: Command<'a>) {
    // TODO: Figure out which command was run and update state
    println!("Command: {:?}", next_command);

    match next_command {
        Command::DoPurchase(improvement) => {
            let result = character.borrow_mut().purchase(improvement);
        }
        Command::StealCurrency => {
            character.borrow_mut().gather_resouce(ResourceType::Income);
        }
        Command::StealFood => {
            character.borrow_mut().gather_resouce(ResourceType::Food);
        }
        cmd => print!("{:?} is not valid in this context", cmd),
    }
}

pub fn render(character: &Character) {
    println!();
    character.print_hud();
}
