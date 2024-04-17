use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;


pub fn create_files() {

    let mut text_to_write = "# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n# ** ** #\n".to_owned();
    
    static TITLE_TEXT: &str = "# ** ** #\n# **                                                                                 Run. ** #";

    if let Ok(lines) = read_lines("./data/test.txt") {
        for line in lines.flatten() {
            let n_line = "                    ".to_string() + &line.to_string()+" \n";
            text_to_write.push_str(&n_line);
        }
        text_to_write.push_str(&TITLE_TEXT);
    }

    let path = Path::new("./data/result.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(text_to_write.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn ascii_creator_create_dir() {
    let path = "./data";
    let my_path = std::path::Path::new(path);

    if my_path.exists() {
        println!("Directory already exists! Skipping creation ...");
        return;
    }

    let create_dir_result: Result<(), std::io::Error> = std::fs::create_dir("./data");
    
    if create_dir_result.is_ok() {
        println!("Created new data dir.");
    }
    else {
        println!("Some problems occurred creating the dir. {:?}", create_dir_result.err());
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}