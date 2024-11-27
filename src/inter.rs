use crate::{
    config::Config,
    err, input_line, lgmain,
    result_trait::{RfocOptionExtended, RfocResultExtended},
};
use colored::Colorize;
use rustyline::DefaultEditor;
use std::{
    env, fs,
    io::{self, Write},
    path::Path,
    process::{self, Stdio},
};

pub fn rfoc_inter(conf: &Config) {
    if conf.interactive.new_buffer {
        println!("\x1b[?1049h");
        println!("\x1b[2");
    }
    let mut rl = DefaultEditor::new().rfoc_unwrap();
    loop {
        let current = env::current_dir().rfoc_unwrap();
        let ps1 = format!("{} > ", current.display());

        match rl.readline(&ps1) {
            Ok(input) => {
                rl.add_history_entry(&input).rfoc_unwrap();

                let vec = input.split_whitespace().collect::<Vec<&str>>();
                if vec.len() == 0{
                    continue;
                }

                match vec[0] {
                    "cd" | "c" => {
                        if vec.len() >= 2 {
                            if let Err(e) = env::set_current_dir(vec[1]) {
                                if e.kind() == io::ErrorKind::NotFound {
                                    println!(
                                        "{}: {}: No such file or directoy",
                                        "Error".red().bold(),
                                        vec[1]
                                    );
                                } else {
                                    eprintln!("{}: {}: {e}", "Error".red().bold(), vec[1]);
                                }
                            }
                        } else {
                            println!("Usage: cd <dir>");
                        }
                        continue;
                    }
                    "ls" | "l" => {
                        ls(current);
                        continue;
                    }
                    "e" | "edit" => {
                        if vec.len() >= 2 {
                            input_line::input_line(vec[1], &conf.syntax.theme)
                        } else {
                            println!("Usage: edit <file>");
                        }
                        continue;
                    }
                    "o" | "open" => {
                        if vec.len() >= 2 {
                            lgmain::read(vec[1], conf.syntax.enable, &conf.syntax.theme);
                        } else {
                            println!("Usage: open <file>");
                        }
                        continue;
                    }
                    "r" | "remove" => {
                        if vec.len() >= 2 {
                            let p: &Path = vec[1].as_ref();
                            if p.is_dir() {
                                println!("Remove the entire contents of the directoy?");
                                let mut buf = String::new();
                                print!("[y/n] ");
                                io::stdout().flush().rfoc_unwrap();

                                io::stdin().read_line(&mut buf).rfoc_unwrap();

                                if buf.trim() == "y" || buf.trim() == "yes" || buf.trim() == "" {
                                    fs::remove_dir_all(p).unwrap_or_else(|e| {
                                        println!("{}: {}", "Error".red().bold(), e);
                                    });
                                    continue;
                                } else {
                                    println!("Removal canceled.");
                                    continue;
                                }
                            }
                            fs::remove_file(vec[1]).unwrap_or_else(|e| {
                                println!("{}: {}", "Error".red().bold(), e);
                            });
                        } else {
                            println!("Usage: remove <file>");
                        }
                    }
                    "s" | "system" => {
                        if vec.len() >= 2 {
                            if let Err(e) = process::Command::new(vec[1])
                                .args(&vec[2..])
                                .stdin(Stdio::inherit())
                                .stdout(Stdio::inherit())
                                .stderr(Stdio::inherit())
                                .status()
                            {
                                if e.kind() == io::ErrorKind::NotFound {
                                    eprintln!(
                                        "{}: {}: Command not found",
                                        "Error".red().bold(),
                                        vec[1]
                                    );
                                    continue;
                                } else {
                                    eprintln!("{}: {}: {}", "Error".red().bold(), vec[1], e);
                                    continue;
                                }
                            }
                            continue;
                        } else {
                            println!("Usage: system <command>");
                        }
                    }
                    "h" | "help" => {
                        help();
                    }
                    "q" | "quit" => {
                        println!("exit");
                        println!("\x1b[?1049l");
                        return;
                    }
                    &_ => {
                        if vec.len() >= 1{
                            println!("Unknown command: {}", vec[1]);
                        }else{
                            continue;
                        }
                    }
                }
            }

            Err(rustyline::error::ReadlineError::Eof)
            | Err(rustyline::error::ReadlineError::Interrupted) => {
                eprintln!("'q' to exit");
            }
            Err(e) => err!(e),
        }
    }
}

#[allow(unused)]
fn ls<P: AsRef<Path>>(dir: P) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let s = entry.path();
                        if s.is_dir() {
                            println!(
                                "{} ",
                                s.file_name()
                                    .rfoc_unwrap()
                                    .to_str()
                                    .rfoc_unwrap()
                                    .blue()
                                    .bold()
                            );
                        } else {
                            println!("{}", s.file_name().rfoc_unwrap().to_str().rfoc_unwrap());
                        }
                    }
                    Err(e) => {
                        err!(e);
                    }
                }
            }
        }
        Err(e) => err!(e),
    }
}

fn help() {
    println!("rfoc interactive mode");
    println!("");
    println!("h(help): display help message");
    println!("");
    println!("o(open): Outputs the contents of the specified file");
    println!("");
    println!("r(remove): remove the specified file");
    println!("");
    println!("q(quit): quit from the rfoc interactive mode");
    println!("");
    println!("c(cd): Moves from the current directory to the specified directory");
    println!("");
    println!("l(ls): Lists all files and directories in the current directory");
    println!("");
    println!("s(system): Executes the specified external command");
}
