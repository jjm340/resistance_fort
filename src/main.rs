use resistance_fort::{process_input, render, update, Context};

fn main() {
    let mut context = Context::new();

    loop {
        process_input(&mut context);
        update(&mut context);
        render(&context);
    }
}
