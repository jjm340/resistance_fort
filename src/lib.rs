pub mod ui {
    use std::error::Error;
    use std::io;
    use std::process;
    use std::str::FromStr;

    pub enum Command {
        Quit,
    }

    impl FromStr for Command {
        type Err = io::Error;

        fn from_str(input: &str) -> Result<Command, Self::Err> {
            match input {
                "quit" => Ok(Command::Quit),
                cmd_text => Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unknown command \"{cmd_text}\""),
                )),
            }
        }
    }

    pub fn input() -> Result<Command, Box<dyn Error>> {
        let mut input = String::new();

        let menu_string = format!(
            r#"Please select an option: 
            1) Move n, s, e, w
            2) Scrounge resources
            3) Attack urban site"#
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

    pub fn process_input() {
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
        }
    }
}
