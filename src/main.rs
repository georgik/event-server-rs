use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use chrono::prelude::*;

const DATA_FILE: &str = "jidelnicek.txt";
const MAX_LEN: usize = 45;

fn split_lines(text: &str, max_len: usize) -> String {
    let mut result = String::new();
    let mut line = String::new();

    for word in text.split_whitespace() {
        if line.is_empty() {
            line.push_str(word);
        } else if line.len() + 1 + word.len() <= max_len {
            line.push(' ');
            line.push_str(word);
        } else {
            result.push_str(&line);
            result.push('\n');
            line.clear();
            line.push_str(word);
        }
    }

    if !line.is_empty() {
        result.push_str(&line);
        // result.push('-');
    }

    result
}


// Find the first bracket in the string and return string before the bracket
fn strip_content_in_bracket(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c == '(' {
            break;
        }
        result.push(c);
    }
    result
}

// Remove trailing whitespace, keep other spaces
fn strip_trailing_whitespace(text: &str) -> String {
    let mut result = String::new();
    let mut last_char = ' ';
    for c in text.chars() {
        if c != ' ' || last_char != ' ' {
            result.push(c);
        }
        last_char = c;
    }
    result
}

fn read_data_from_file() -> String {
    // Open file for reading
    let file = File::open(DATA_FILE).unwrap();
    let reader = BufReader::new(file);

    // Get current date
    let today = Local::today().format(" %-d.%-m.").to_string();

    // Find first line with current date
    let mut found_date = false;
    let mut output = String::new();
    let mut full_content = String::new();
    println!("{}", today);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&today) {
            found_date = true;
        }
        let line = strip_content_in_bracket(&line);
        let line = strip_trailing_whitespace(&line);
        let line = split_lines(&line, MAX_LEN);
        if found_date {
            output += &line;
            output.push('\n');
        }
        full_content += &line;
        full_content.push('\n');
    }

    if !found_date {
        return full_content;
    }

    output
}

// write string to file
fn write_to_file(text: &str) {
    let mut file = File::create(DATA_FILE).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

fn main() {
    let output = read_data_from_file();
    // Print output
    // println!("{}", output);
    write_to_file(&output);
}

