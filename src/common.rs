use std::io::{Read, Result};
use std::fs::File;

pub fn load_input(day: &str) -> Result<String> {
    let path = format!("resources/{}.txt", day);

    let mut input = String::new();

    let mut file = File::open(path)?;

    file.read_to_string(&mut input)?;

    Ok(input)
}
