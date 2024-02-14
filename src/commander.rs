use crate::process::{read_file, output_of_vec, replace_vec};
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;
pub fn save_file(input_path: &str, lines: &Vec<String>) -> io::Result<()> {
    let path = Path::new(input_path);
    let directory = path.parent().unwrap();
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let new_file_name = format!("{}_reformatted.srt", stem);
    let new_path = directory.join(new_file_name);

    let mut file = File::create(new_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
fn open_file() -> io::Result<(Vec<String>, String)> {
    print!("input absolute file path > ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove leading and trailing whitespaces
    if input.to_lowercase().as_str() == "q" {
        println!("You don't want to open a file.\nProgram will be terminated.");
        std::process::exit(0);
    }
    let first_char = input.chars().next();
    let last_char = input.chars().last();

    let file_path = if first_char == Some('\"') && last_char == Some('\"') {
        // If the input is surrounded by quotes, remove them
        format!(r"{}", &input[1..input.len()-1])
    } else {
        // If the input is not surrounded by quotes, use it as is
        format!(r"{}", input)
    };

    let lines = read_file(&file_path)?;

    Ok((lines, file_path))
}
pub fn commander() {
    crate::process::set_debug(false);
    loop {
        #[allow(unused_assignments)]
        let mut is_success_replace: bool = false;
        let debug = crate::process::get_debug();
        match open_file() {
            Ok((mut lines, file_path)) => {
                println!("Successfully opened file. \u{25A0}");
                // Do something with lines
                if debug {
                    output_of_vec(&lines, 150);
                    println!("------------------------------------------------------------");
                }
                print!("Do you want to make a reformatted file? (y/n) > ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                match input.trim().to_lowercase().as_str() {
                    "y" => {
                        replace_vec(&mut lines);
                        is_success_replace = true;
                        if debug {
                            println!("output of lines:");
                            output_of_vec(&lines, 200);
                            println!("------------------------------------------------------------");
                        }
                    },
                    _ => {
                        println!("You don't want to make a reformatted file.");
                    }
                }
                if is_success_replace {
                    print!("Do you want to save the reformatted file? (y/n) > ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    match input.trim().to_lowercase().as_str() {
                        "y" => {
                            match save_file(&file_path, &lines) {
                                Ok(_) => println!("Successfully saved the reformatted file. \u{25A0}"),
                                Err(e) => println!("Failed to save the reformatted file: {} \u{25A1}", e),
                            }
                        },
                        _ => {
                            println!("You don't want to save the reformatted file.");
                        }
                    }
                }
            },
            Err(e) => {
                println!("Failed to open file: {} \u{25A1}", e);
                // Continue the loop to prompt the user again
            }
        }
    }
}