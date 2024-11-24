use std::{error::Error, process};

pub fn log_and_exit(e: Option<Box<dyn Error>>) -> ! {
    if let Some(err) = e {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }

    print!("Exiting normally...");
    process::exit(0);
}
