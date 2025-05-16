use std::io::{self, Write};

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erreur de lecture");
    buffer.trim().to_string()
}

pub fn welcome_message() {
    println!("Salut, je suis Spark. Tape /exit pour quitter.");
}
