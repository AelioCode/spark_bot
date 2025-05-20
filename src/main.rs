// main.rs

/// Code principal du projet
/// Ne sert pour l'instant qu'à lancer la boucle principale spark_bot

mod core;
mod bot;
mod commands;

fn main() {
    bot::spark_bot();
}
