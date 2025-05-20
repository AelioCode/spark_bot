// core/input.rs

/// Fonctions de communications avec l'utilisateur
/// get_input
/// welcome_message

use std::io::{self};

// récupère le texte donné par l'utilisateur
pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erreur de lecture");
    buffer.trim().to_string()
}

// Message de bienvenue
pub fn welcome_message() { 
    println!("Salut, je suis Spark. Tape /exit pour quitter.");
}
