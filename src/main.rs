use resistance_fort::{process_input, render, update, Context};

fn main() {
    let mut context = Context::new();

    loop {
        let next_command = process_input(&mut context);
        update(&mut context, next_command);
        render(&context);
    }
}
