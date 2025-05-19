// bot.rs

/// Cerveau de Spark

use std::collections::HashMap;

use crate::core::input::{
    get_input,
    welcome_message
};

use crate::commands::{
    help, localize, pomodoro, recall, remember, remind, start, todo
};


use crate::core::command_info::Command;

use crate::core::context::Context;

pub fn spark_bot() {

    let mut todo_commands: HashMap<&str, &str> = HashMap::new();
    todo_commands.insert("/new", "Créer une nouvelle liste");
    todo_commands.insert("/show","Afficher les éléments de la liste");
    todo_commands.insert("/edit", "Permet d'éditer une liste");
    todo_commands.insert("/exit","Quitter le gestionnaire de liste");

    let mut ctx = Context {
        memory: String::new(),
        todo_list: HashMap::new(),
        reminders: Vec::new()
    };

    welcome_message();

    loop {
        let input = get_input();

        match Command::from(input.as_str()) {
            Command::Start => start::handle_start(),
            Command::Remember => remember::handle_remember(&mut ctx),
            Command::Recall => recall::handle_recall(&ctx.memory),
            Command::Pomodoro => pomodoro::handle_pomodoro(),
            Command::Localize => localize::handle_localize(),
            Command::Todo => todo::handle_todo(&mut ctx.todo_list),
            Command::Remind => remind::handle_remind(&mut ctx, input.as_str()),
            Command::Help => help::handle_help(),
            Command::Exit => {
                println!("À bientôt !");
                break;
            }
            Command::Unknown => println!("Commande inconnue !"),
        }
    }
}
