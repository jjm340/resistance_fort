pub mod ui {
    use std::error::Error;
    use std::io;
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

    pub fn menu_next_command() -> Result<Command, Box<dyn Error>> {
        let mut input = String::new();

        if let Err(e) = io::stdin().read_line(&mut input) {
            Err(Box::new(e))
        } else {
            let parsed = Command::from_str(&input);

            // if let Ok(cmd) = Command::from_str(&input) {
            //     Ok(cmd)
            // } else {

            // }

            match parsed {
                Ok(cmd) => Ok(cmd),
                Err(not_recognized) => Err(Box::new(not_recognized)),
            }
        }
    }
}
