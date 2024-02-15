use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref DEBUG01: Mutex<bool> = Mutex::new(false);
}
pub fn set_debug01(value: bool) {
    let mut debug = DEBUG01.lock().unwrap();
    *debug = value;
}
pub fn get_debug01() -> bool {
    let debug = DEBUG01.lock().unwrap();
    *debug
}

pub fn read_file(input_path: &String) -> io::Result<Vec<String>> {
    let path = Path::new(input_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }
    Ok(lines)
}

pub fn output_of_vec(lines: &Vec<String>, count: usize) {
    if count > lines.len() {
        println!("count is bigger than lines.len()");
        return;
    }
    println!("lines = {}", lines.len());
    for i in 0..count {
        println!("{}", lines[i]);
    }
}

pub fn fix_sub_vec(lines: &mut Vec<String>) -> io::Result<()> {
    let debug = get_debug01();
    let year_reg = Regex::new(r"^(\d{4,5})(년)$").unwrap();
    for line in lines.iter_mut() {
        if let Some(caps) = year_reg.captures(line) {
            if let Some(mat) = caps.get(1) {
                if debug { println!("{} -> year", mat.as_str().to_string()) }
                *line = mat.as_str().to_string();
            }
        }
    }
    if debug {
        for line in lines.iter() {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn replace_vec(lines: &mut Vec<String>) {
    let debug01 = DEBUG01.lock().unwrap();
    let reg_time_line = Regex::new(r"(^\d{2}:\d{2}:\d{2},\d{3} --> \d{2}:\d{2}:\d{2},\d{3}$)")
        .unwrap();
    let normal_case = Regex::new(r"(^\d+$)").unwrap();
    let end_case = Regex::new(r"(\d+$)").unwrap();

    let mut i = 0;
    while i < lines.len() {
        if reg_time_line.is_match(&lines[i]) {
            if *debug01 {
                println!("{} -> time", lines[i]);
            }
        } else if normal_case.is_match(&lines[i]) {
            match lines[i].parse::<u32>() {
                Ok(number) => { if *debug01 {
                    println!("{} -> normal, Number: {}", lines[i], number) } },
                Err(_) => println!("{} -> Failed to parse number \u{25A1}", lines[i]),
            }
        } else if let Some(caps) = end_case.captures(&lines[i]) {
            if let Some(mat) = caps.get(1) {
                match mat.as_str().parse::<u32>() {
                    Ok(number) => { if *debug01 {
                        println!("{} -> end, Number: {}", lines[i], number) } },
                    Err(_) => println!("{} -> Failed to parse number \u{25A1}", lines[i]),
                }
                let (subtitle, order_number) = lines[i].split_at(mat.start());
                // Replace the line at index i with order_number and subtitle.trim()
                lines.splice(i..i+1, vec![subtitle.trim().to_string(),
                                          String::from(""), order_number.to_string()]);
                i += 1; // Skip the newly added line
            }
        } else {
            if *debug01 { println!("{} -> no match", lines[i]) };
            if i < lines.len() - 1 {
                lines.insert(i + 1, String::from(""));
                i += 1; // Skip the newly added line
            }
        }
        i += 1;
    }
    println!("successfully reformatted. \u{25A0}");
}