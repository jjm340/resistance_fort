use resistance_fort::ui::menu_next_command;

fn main() {
    loop {
        let next_command = menu_next_command();

        if let Err(e) = run(config) {
            println!("Application error: {e}");
            process::exit(1);
        }
        println!("again!");
    }
}
