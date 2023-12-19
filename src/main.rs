use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), std::io::Error>{

    let tl_path = "sona/translations/nl/definitions.toml";
    let tl_file = File::open(tl_path)?;
    let tl_reader = io::BufReader::new(tl_file);

    let mut tl_dictionary: HashMap<String, String> = HashMap::new();

    for line in tl_reader.lines() {
        let line_content = line?;
        // Assuming the file has key-value pairs separated by a delimiter (e.g., '=')
        let parts: Vec<&str> = line_content.split('=').collect();

        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            //println!("{} {}", key, value);
            tl_dictionary.insert(key, value);
        } else {
            eprintln!("Invalid line: {}", line_content);
        }
    }


    let input_string = "mi olin e sina.";
    let input_string = "jan pona mi li lon tomo pali. jan jo pi tomo pali li pana e tomo lape tawa mi tu. tenpo pimeja pi ale ala la mi tu li lon tomo ni. lape la ilo kon li pona mute tawa ona. taso mi  sama ala ona. taso mi toki ala ike tawa ona.";

    let directory_path = "sona/words/";
    let entries = fs::read_dir(directory_path)?;

    for word in input_string.split(|c: char| !c.is_alphanumeric()) {
        if !word.is_empty() {
            //println!("{}", word);

            if tl_dictionary.contains_key(word) {
                println!("{} -> {}", word, tl_dictionary[word]);
            } else {
                println!("{} -> ??", word);
            }
            /*entries.for_each(|e| {

            });
            for entry in &entries {
                let dir_entry = e.unwrap();
                let file_name = dir_entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.contains(word) {
                    println!("-aa");
                }
            }*/

        }
    }

    Ok(())
}
