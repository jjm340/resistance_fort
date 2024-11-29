pub mod character_record;
pub mod commands;
pub mod improvements;
pub mod input;
pub mod menu;
pub mod utils;

use commands::Command;
use improvements::ImprovementCollection;
use menu::main_menu;

use crate::character_record::Character;
use core::panic;
use std::fmt;

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
// Tells you about the current situation in the game world
pub struct Context<'a> {
    character_record: Character<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            character_record: Character::new(),
        }
    }
}

pub fn process_input<'a>(
    _context: &mut Context,
    improvement_collection: &'a ImprovementCollection,
) -> Command<'a> {
    match main_menu(improvement_collection) {
        Some(cmd) => cmd,
        _ => {
            panic!("The main menu produced no valid command");
        }
    }
}

pub fn update<'a>(_context: &mut Context, next_command: Command<'a>) {
    // TODO: Figure out which command was run and update state
}

pub fn render(context: &Context) {
    println!();

    context.character_record.print_hud();
}
