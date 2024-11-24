use std::io::{self};

use crate::utils::log_and_exit;

pub fn fetch_input() -> String {
    let mut input = String::new();

    if let Err(e) = io::stdin().read_line(&mut input) {
        log_and_exit(Some(Box::new(e)));
    }

    let cleaned_input = input.to_lowercase();
    let cleaned_input = cleaned_input.trim();
    cleaned_input.to_string()
}
