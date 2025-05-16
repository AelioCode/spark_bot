use crate::core::command_info::all_commands;

pub fn handle_help() {
    println!("📘 Commandes disponibles :");
    for cmd in all_commands() {
        println!("{:<12}→ {}", cmd.name, cmd.description);
    }
}