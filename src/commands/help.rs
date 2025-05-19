// commands/help.rs

/// Afficher la liste des commandes

use crate::core::command_info::all_commands;

//fonction handle
pub fn handle_help() {
    println!("📘 Commandes disponibles :");

    //Affiche toutes les commandes de all_commands dans command_info.rs
    for cmd in all_commands() {
        println!("{:<12}→ {}", cmd.name, cmd.description);
    }
}