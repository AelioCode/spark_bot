


// use reqwest::blocking::get;
// use serde::Deserialize;

use crate::core::input::{
    get_input,
    welcome_message
};

use crate::core::commands::{
    start,
    recall,
    help,
    pomodoro,
    localize
};

use crate::core::todo::{
    todo,
};

pub fn spark_bot() {
    let mut memory = String::new();

    let mut commands: HashMap<&str, &str> = HashMap::new();
    commands.insert("/start", "Démarrer une nouvelle tâche");
    commands.insert("/remember", "Mémoriser une information");
    commands.insert("/recall", "Afficher ce que j'ai mémorisé");
    commands.insert("/help", "Afficher cette aide");
    commands.insert("/exit", "Quitter le programme");
    commands.insert("/pomodoro", "Lancer un minuteur Pomodoro");
    commands.insert("/localize", "Me localiser dans le monde");
    commands.insert("/todo", "gérer une liste de tâches");

    let mut todo_commands: HashMap<&str, &str> = HashMap::new();
    todo_commands.insert("/new", "Créer une nouvelle liste");
    todo_commands.insert("/show","Afficher les éléments de la liste");
    todo_commands.insert("/edit", "Permet d'éditer une liste");
    todo_commands.insert("/exit","Quitter le gestionnaire de liste");

    let mut todo_list: HashMap<String, Vec<String>> = HashMap::new();


    welcome_message();

    loop {
        let input = get_input();

        match input.as_str() {
            "/start" => start(),
            "/remember" => {
                println!("Que dois-je me souvenir ?");
                memory = get_input();
                println!("Ok, je m'en souviendrai !");
            }
            "/recall" => recall(&memory),
            "/pomodoro" => pomodoro(),
            "/localize" => localize(),
            "/todo" => todo(&mut todo_list),


            "/exit" => {
                println!("À bientôt !");
                break;
            }
            "/help" => help(&commands),
            _ => println!("Commande inconnue !"),
        }
    }
}

