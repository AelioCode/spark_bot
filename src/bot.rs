use std::collections::HashMap;

use crate::core::input::{
    get_input,
    welcome_message
};

use crate::core::commands::{
    start,
    recall,
    help,
    pomodoro,
    localize,
    Command,
};

use crate::core::todo::{
    todo,
};

struct Context {
        memory: String,
        todo_list: HashMap<String, Vec<String>>,
    }


pub fn spark_bot() {

    let mut todo_commands: HashMap<&str, &str> = HashMap::new();
    todo_commands.insert("/new", "Créer une nouvelle liste");
    todo_commands.insert("/show","Afficher les éléments de la liste");
    todo_commands.insert("/edit", "Permet d'éditer une liste");
    todo_commands.insert("/exit","Quitter le gestionnaire de liste");

    let mut ctx = Context {
        memory: String::new(),
        todo_list: HashMap::new(),
    };



    welcome_message();

    loop {
        let input = get_input();

        match Command::from(input.as_str()) {
            Command::Start => start(),
            Command::Remember => {
                println!("Que dois-je me souvenir ?");
                ctx.memory = get_input();
                println!("Ok, je m'en souviendrai !");
            }
            Command::Recall => recall(&ctx.memory),
            Command::Pomodoro => pomodoro(),
            Command::Localize => localize(),
            Command::Todo => todo(&mut ctx.todo_list),
            Command::Help => help(),
            Command::Exit => {
                println!("À bientôt !");
                break;
            }
            Command::Unknown => println!("Commande inconnue !"),
        }
    }
}
