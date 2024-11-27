#![deny(warnings)]
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_and_line<T: AsRef<Path>>(path: T) -> Result<(), Box<dyn Error>> {
    let mut line: u64 = 0;

    let mut vec: Vec<String> = Vec::new();

    for result in BufReader::new(File::open(path.as_ref())?).lines() {
        line += 1;
        let result = result?;

        if line >= 10 && line <= 99 {
            let s = format!("{line}  |   {result}");
            vec.push(s);
        } else if line >= 100 && line <= 999 {
            let a = format!("{line} |  {result}");
            vec.push(a);
        } else if line >= 1000 {
            let t = format!("{line}| {result}");
            vec.push(t);
        } else {
            let st = format!("{line}   |    {result}");
            vec.push(st);
        }
    }

    vec.iter().for_each(|f| println!("{f}"));

    Ok(())
}
