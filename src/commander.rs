use crate::process::{fix_sub_vec, output_of_vec, read_file, replace_vec};
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;
enum Type {
    Reformat,
    Fix,
}

fn save_file(input_path: &str, lines: &Vec<String>, fix_type: Type) -> io::Result<()> {
    let path = Path::new(input_path);
    let directory = path.parent().unwrap();
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let new_file_name = match fix_type {
        Type::Reformat => format!("{}{}", stem, "_reformatted.srt"),
        Type::Fix => format!("{}{}", stem, "_fixed.srt"),
    };
    let new_path = directory.join(new_file_name);

    let mut file = File::create(new_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_file_path() -> io::Result<String> {    
    let input = get_input("file path > ");

    if input == "q" {
        println!("You don't want to open a file.\nProgram will be terminated.");
        std::process::exit(0);
    }
    // 콤마가 입력된 경우 콤마를 제거하기 위해 입력된 String 양 쪽을 얻는다.
    let first_char = input.chars().next();
    let last_char = input.chars().last();

    let file_path = 
    if first_char == Some('\"') && last_char == Some('\"') || 
    first_char == Some('\'') && last_char == Some('\'') {
        // If the input is surrounded by quotes, remove them
        format!(r"{}", &input[1..input.len()-1])
    } else {
        // If the input is not surrounded by quotes, use it as is
        format!(r"{}", input)
    };

    let path = Path::new(&file_path);
    // canonicalize() 함수를 통해 상대 경로를 절대 경로로 변환한다.
    let absolute_path = path.canonicalize()?;
    // to_string_lossy() 함수는 절대 경로를 유효한 UTF-8 문자열로 변환하고, to_string()을 통해
    // 함수의 결과를 String 타입으로 변환한다.
    // to_string_lossy() 함수는 유효하지 않은 유니코드 문자가 있을때에는 �로 변환하고 Owned 타입을 반환한다.
    // 유효한 유니코드 문자가 있을때에는 Borrowed 타입을 반환한다.
    Ok(absolute_path.to_string_lossy().to_string())
}

fn open_and_read_file() -> io::Result<(Vec<String>, String)> {
    let file_path = get_file_path()?;    
    let lines = read_file(&file_path)?;
    Ok((lines, file_path))
}

fn reformat_process() {
    #[allow(unused_assignments)]
    let mut is_success_replace: bool = false;
    let debug01 = crate::process::get_debug01();
    match open_and_read_file() {
        Ok((mut lines, file_path)) => {
            println!("Successfully opened file. \u{25A0}");
            // Do something with lines
            if debug01 {
                output_of_vec(&lines, 150);
                println!("------------------------------------------------------------");
            }
            let input = 
                get_input("Do you want to make a reformatted file? (y/n) > ");            
            match input.trim().to_lowercase().as_str() {
                "y" => {
                    replace_vec(&mut lines);
                    is_success_replace = true;
                    if debug01 {
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
                let input = 
                    get_input("Do you want to save the reformatted file? (y/n) > ");
                match input.trim().to_lowercase().as_str() {
                    "y" => {
                        match save_file(&file_path, &lines, Type::Reformat) {
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

fn fix_sub_process() {
    let mut is_success_fix: bool = false;
    //let debug = crate::process::get_debug();
    match open_and_read_file() {
        Ok((mut lines, file_path)) => {
            println!("Successfully opened file. \u{25A0}");
            let input = get_input("Do you want to fix the file? (y/n) > ");
            if input == "y" {
                if let Ok(()) = fix_sub_vec(&mut lines) {
                    is_success_fix = true;
                    println!("Successfully fixed the file. \u{25A0}");
                }
            } else {
                println!("You don't want to fix the file.");
            }
            if is_success_fix {
                let input = 
                    get_input("Do you want to save the fixed file? (y/n) > ");
                match input.trim().to_lowercase().as_str() {
                    "y" => {
                        match save_file(&file_path, &lines, Type::Fix) {
                            Ok(_) => println!("Successfully saved the fixed file. \u{25A0}"),
                            Err(e) => println!("Failed to save the fixed file: {} \u{25A1}", e),
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
        }
    }
}

pub fn commander() {
    loop {
        let command = get_input("cmd> ");
        match command.as_str() {
            "reformat sub" => reformat_process(),
            "fix sub" => fix_sub_process(),
            "exit" => {
                println!("exit program");
                break;
            }
            _ => (),
        }
    }
}