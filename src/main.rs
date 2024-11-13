use std::io::{self, BufRead, Write};
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;

fn main() -> io::Result<()> {
    let path = "/mnt/5CD266C7D266A54C/Miscellaneous Files/Personal Programming/Rust Programs/project1/src/words.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut matches = String::new();

    print!("Enter a combination of letters: ");
    io::stdout().flush()?; 

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let search_str = input.trim().to_lowercase();

    if search_str.is_empty() {
        println!("Invalid input. Please enter a non-empty string.");
        return Ok(());
    }

    let search_char_count = build_char_count(&search_str);

    for line in reader.lines() {
        let line = line?.trim().to_lowercase();

        if line.len() != search_str.len() {
            continue;
        }

        if build_char_count(&line) == search_char_count {
            matches.push_str(&line);
            matches.push('\n');
        }
    }

    println!("{}", matches);
    Ok(())
}

fn build_char_count(s: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();
    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    char_count
}