// use std::io::{self};
// use std::process;

use resistance_fort::{
    // character_record::{self, load_improvements, Character, Improvement},
    improvements::{self, ImprovementCollection},
    process_input,
    render,
    update,
    Context,
};

fn main() {
    let improvement_collection = ImprovementCollection::new();
    let mut context = Context::new();

    loop {
        let next_command = process_input(&mut context, &improvement_collection);
        update(&mut context, next_command);
        render(&context);
    }
}
