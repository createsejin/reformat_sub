mod process;
mod commander;
use std::env;

use crate::process::set_debug01;

fn main() {
    println!("Reformation of Papa go's translated text into a readable srt format");

    let args: Vec<String> = env::args().collect();

    for arg in &args {
        if arg.starts_with("--debug01=") {
            let value: Vec<&str> = arg.split('=').collect();            
            if value.len() == 2 {
                set_debug01(value[1] == "true");
            }
        }
    }

    commander::commander();
}
