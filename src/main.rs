use resistance_fort::ui::*;
use std::process;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const SIXTEEN_MILLIS: Duration = Duration::from_millis(16);

fn main() {
    loop {
        let start = SystemTime::now();

        process_input();
        // update();
        // render();

        let elapsed = SystemTime::now().duration_since(start + SIXTEEN_MILLIS);

        match elapsed {
            Err(e) => {
                eprintln!("Error computing elapsed time in game loop: {}", e);
                process::exit(1)
            }
            Ok(elapsed) => {
                sleep(elapsed);
            }
        }
    }
}
