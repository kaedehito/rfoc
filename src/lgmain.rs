#![deny(warnings)]
use crate::result_trait::{RfocOptionExtended, RfocResultExtended};
use std::fs;
use std::io;
use std::path::Path;
use colored::Colorize;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use syntect::util::LinesWithEndings;

pub fn read<T: AsRef<Path>>(path: T, hig: bool, theme_name: &str) {
    let path: &Path = path.as_ref();
    let mut error = false;

    let extensition = path.extension();
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound{
            eprintln!("{}: No such file or directory", "Error".red().bold());
            error = true;
            String::from("##__ERROR__##")
        }else{
            eprintln!("{}: {}", "Error".red().bold(), e);
            error = true;
            String::from("##__ERROR__##")
        }
    });

    if error {
        return;
    }

    match extensition {
        Some(o) => {
            if hig {
                let extens = o.to_str().rfoc_unwrap();

                // setting up for syntax higlight
                let ps = SyntaxSet::load_defaults_newlines();
                let ts = ThemeSet::load_defaults();

                if let Some(syntax) = ps.find_syntax_by_extension(extens) {
                    let theme = &ts.themes[theme_name];
                    let mut h = HighlightLines::new(syntax, theme);

                    // display the higlight text
                    for line in LinesWithEndings::from(content.as_str()) {
                        let ranges = h.highlight_line(line, &ps).rfoc_unwrap();
                        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                        print!("{}", escaped);
                    }
                } else {
                    println!("{content}");
                }
            } else {
                println!("{content}");
            }
        }
        None => {
            println!("{content}")
        }
    }
}

pub fn highlight_help() {
    let ts = ThemeSet::load_defaults();
    let mut themes: Vec<&String> = ts.themes.keys().collect();
    themes.sort();

    println!("-------");
    for (i, theme_name) in themes.iter().enumerate() {
        println!("{}. {}", i + 1, theme_name);
    }

    println!("-------\nall {} themes", themes.len());
}
