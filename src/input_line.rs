#![deny(warnings)]

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use clap::builder::OsStr;
use colored::Colorize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::{err, syntax};
use crate::result_trait::RfocResultExtended;

pub fn input_line<P: AsRef<Path>>(path: P, theme: &str) {
    let path = path.as_ref();

    let nul = &OsStr::from("");
    let path_ex = path.extension().unwrap_or_else(|| {
        nul
    });
    let extension = path_ex.to_str().unwrap();
    let mut create = false;

    let file = File::open(path).unwrap_or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound{
            eprintln!("File not found. Create new file");
            create = true;
            return File::create(path).rfoc_unwrap();
        }else{
            err!(e);
        }
    });
    let mut buffer: Vec<String> = Vec::new();

    if !create {
        let reader = BufReader::new(file);
        buffer = reader.lines().map(|l| l.expect("Failed to read line")).collect();
    }

    let mut rl = DefaultEditor::new().expect("Failed to initialize editor");
    let mut insert_mode = false;
    let mut insert_add = false;
    let mut current_line = 0; 

    loop {
        let ps1 = if insert_mode { "" } else { "> " };

        match rl.readline(ps1) {
            Ok(input) => {
                rl.add_history_entry(&input).unwrap_or_else(|_| {
                    println!("{}: Failed to add history", "Error".red().bold());
                    false
                });

                if insert_mode {
                    if input == "." {
                        insert_mode = false;
                    } else {
                        if insert_add{
                            let add_pos = current_line + 1;
                            if add_pos <= buffer.len(){
                                buffer.insert(current_line, input.clone());
                            }else if add_pos == buffer.len(){
                                buffer.push(input.clone());
                            }else {
                                buffer.push(input.clone());
                            }
                            current_line = add_pos;
                            continue;
                        }
                        buffer.insert(current_line, input); 
                        current_line += 1;
                    }
                } else {
                    match input.as_str() {
                        "p" => {
                            let text = buffer.join("\n");
                            let syntax = syntax::enable_syntax(&text, extension, theme);
                            let mut sum = 0 as u64;
                            let _ = &syntax.lines().into_iter().for_each(|f| {
                                sum += 1;
                                println!("{sum}: {f}");
                            });
                        }
                        "d" => {
                            if current_line < buffer.len() {
                                buffer.remove(current_line);
                                println!("Line {} deleted.", current_line + 1);
                            } else {
                                println!("No line to delete.");
                            }
                        }
                        "q" => {
                            println!("Exit editor mode.");
                            break;
                        }
                        "w" => {
                            let mut file = File::create(path).expect("Failed to open file for writing");
                            for line in &buffer {
                                writeln!(file, "{}", line).expect("Failed to write to file");
                            }
                            println!("Save to file");
                        }
                        "i" => {
                            insert_mode = true;
                            insert_add = false;
                        }
                        "a" => {
                            insert_mode = true;
                            insert_add = true;
                        }
                        n if n.parse::<usize>().is_ok() => {
                            let line_num = n.parse::<usize>().unwrap();
                            if line_num > 0 && line_num <= buffer.len() {
                                current_line = line_num - 1;
                                println!("Moved to line {}.", current_line + 1);
                            } else {
                                println!("Invalid line number.");
                            }
                        }
                        _ => {
                            println!("Unknown command: {}", input);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Type 'q' to quit.");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    // 終了時にファイルに書き戻し
    let mut file = File::create(path).expect("Failed to open file for writing");
    for line in &buffer {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
}

