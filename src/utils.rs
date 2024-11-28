use std::{fmt::Display, process};

pub fn log_and_exit(e: Option<Box<dyn Display>>) -> ! {
    if let Some(err) = e {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }

    print!("Exiting normally...");
    process::exit(0);
}

pub fn wrap_error<T: Display + 'static>(e: T) -> Option<Box<dyn Display>> {
    Some(Box::new(e))
}
