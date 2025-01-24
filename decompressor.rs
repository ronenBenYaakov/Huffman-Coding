use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::string;
use serde_json::Value;

fn decompressor() -> io::Result<()> {
    // Open the file
    let file = File::open("src\\example")?;
    let reader = BufReader::new(file);

    // Read the lines from the file into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // Parse the second line as JSON into a HashMap
    let parsed: HashMap<String, String> = serde_json::from_str(&lines[1]).expect("Failed to parse JSON");

    // Get the binary text from the first line
    let binary_txt = &lines[0];

    let mut s = String::new();
    let mut real_text = String::new();

    // Iterate through the binary text and decode
    for c in binary_txt.chars() {
        s.push(c);

        // Find the corresponding key for the binary string
        if let Some(key) = parsed.iter().find_map(|(k, v)| if v == &s { Some(k) } else { None }) {
            real_text.push_str(key);
            s.clear();  // Reset the temporary binary string
        }
    }

    // Print the decoded text
    println!("Decoded text: {}", real_text);

    Ok(())
}

fn main() {
    if let Err(e) = decompressor() {
        eprintln!("Error: {}", e);
    }
}
