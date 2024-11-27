use std::{fs::File, io::{BufRead, BufReader}, path::Path};


pub fn find<P: AsRef<Path>>(file: P, find: &str) -> Result<(), Box<dyn std::error::Error>>{
    let path: &Path = file.as_ref();

    let mut num = 1 as u64;
    for result in BufReader::new(File::open(path)?).lines(){
        let result = result.unwrap();

        if let Some(_) = result.find(find){
            let p = result.replace(find, &format!("\x1b[31m{find}\x1b[0m"));
            println!("{num}: {p}");
        }
        num += 1;
        
    }
    
    Ok(())
}
