#![deny(warnings)]
use std::fs;
use clap::Parser;
use colored::*;
use input_line::input_line;
use result_trait::RfocResultExtended;
mod config;
mod lgmain;
mod line;
mod result_trait;
mod input_line;
mod syntax;
mod find;
mod inter;

#[derive(Parser)]
#[command(name = "rfoc")]
#[command(author = "ogasawara futo")]
#[command(version = "1.0")]
#[command(about = "Light File Open program", long_about = None)]
struct Cli {
    /// The file to read
    #[arg(required_unless_present = "themes", required_unless_present = "interactive")]
    pub file: Option<String>,
    
    /// Outputs the contents of a file with line numbers (maximum supported: 9999 lines)
    #[arg(short, long)]
    pub line: bool,

    /// Reads input from standard input and appends it to the specified file
    #[arg(short, long)]
    pub edit: bool,

    /// Remove the file
    #[arg(short, long)]
    pub remove: bool,

    /// Outputs the locations of the specified string within the contents of the file.
    #[arg(short, long)]
    pub find: bool,

    /// The string to search for (required when --find is enabled)
    #[arg(requires = "find")]
    pub findtext: Option<String>,

    /// Lists all enabled syntax highlighting themes
    #[arg(long)]
    pub themes: bool,

    /// Starts rfoc interactively
    #[arg(long, short)]
    pub interactive: bool,
}

fn main() {
    let conf = config::get_config();
    let cli = Cli::parse();

    let edit = cli.edit;
    let remove = cli.remove;
    let line = cli.line;
    let find = cli.find;
    let inter = cli.interactive;

    if cli.themes {
        lgmain::highlight_help();
        return;
    }

    if inter{
        inter::rfoc_inter(&conf);
        return;
    }

    let file = cli.file.unwrap_or_else(|| {
        eprintln!("{}: No such file or directory", "Error".red().bold());
        std::process::exit(1);
    });

    if find {
        let f = cli.findtext.unwrap_or_else(|| {
            eprintln!("{}: The string to search for is required", "Error".red().bold());
            std::process::exit(1);
        });
        find::find(&file, &f).unwrap_or_else(|e| {
            err!(e);
        });
        return;
    }

    if remove{
        fs::remove_file(&file).rfoc_unwrap();
        return;
    }

    if line {
        line::read_and_line(&file).unwrap_or_else(|e| {
            eprintln!("{}: {e}", "Error".red().bold());
            std::process::exit(1);
        });
        return;
    }

    if edit{
        input_line(&file, &conf.syntax.theme);
        return;
    }


    lgmain::read(&file, conf.syntax.enable, &conf.syntax.theme);
    std::process::exit(0);
}
