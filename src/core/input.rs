use std::io::{self, Write};

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erreur de lecture");
    buffer.trim().to_string()
}