pub mod character_record;
pub mod improvement;

use crate::character_record::Character;
use std::error::Error;
use std::io;
use std::process;
use std::str::FromStr;

pub enum Command {
    Quit,
    Purchase,
}

// Tells you about the current situation in the game world
pub struct Context {
    character_record: Character,
    current_status: String,
}

impl Context {
    pub fn new() -> Context {
        Context {
            character_record: Character::new(),
            current_status: String::from("Open"),
        }
    }
}

impl FromStr for Command {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input.trim() {
            "q" | "quit" => Ok(Command::Quit),
            "1" | "purchase" => Ok(Command::Purchase),
            cmd_text => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown command \"{cmd_text}\""),
            )),
        }
    }
}

fn input() -> Result<Command, Box<dyn Error>> {
    let mut input = String::new();

    let menu_string = format!(
        r#"
    Please select an option: 
    1) Purchase improvement
    2) Steal food
    3) Steal currency
"#
    );

    print!("{menu_string}");

    if let Err(e) = io::stdin().read_line(&mut input) {
        Err(Box::new(e))
    } else {
        let parsed = Command::from_str(&input);

        match parsed {
            Ok(cmd) => Ok(cmd),
            Err(not_recognized) => Err(Box::new(not_recognized)),
        }
    }
}

pub fn process_input(context: &mut Context) {
    let next_command = input();

    match next_command {
        Err(e) => {
            eprintln!("Error fetching next command: {}", e);
            process::exit(1);
        }
        Ok(Command::Quit) => {
            println!("Exiting application...");
            process::exit(0);
        }
        Ok(Command::Purchase) => {
            print!("Purchase a thing!");
        }
    }
}

pub fn update(context: &mut Context) {}

pub fn render(context: &Context) {
    println!();

    context.character_record.print_hud();
}
