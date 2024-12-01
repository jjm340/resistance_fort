use resistance_fort::{character_record::Character, process_input, render, update};
use std::cell::RefCell;

fn main() {
    let character = RefCell::new(Character::new());
    character.borrow().print_hud();

    loop {
        let next_command = process_input();

        // let mut mut_context =
        update(&character, next_command);
        render(&character.borrow());
    }
}
