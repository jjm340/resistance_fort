use std::io::{self};
use std::process;

use resistance_fort::{
    character_record::{self, load_improvements, Character, Improvement},
    process_input, render, update, Context,
};

fn log_and_exit(e: io::Error) -> ! {
    eprintln!("Fatal error occurred: {}", e);
    process::exit(1);
}

fn main() {
    let mut context = Context::new();

    let improvements = load_improvements();
    let improvements = match improvements {
        Ok(improvements) => improvements,
        Err(e) => log_and_exit(e),
    };

    println!("improvements: {:?}", improvements);

    loop {
        process_input(&mut context);
        update(&mut context);
        render(&context);
    }
}
