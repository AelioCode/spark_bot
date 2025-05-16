use std::io::{self, Write};

use reqwest::blocking::get;
use serde::Deserialize;


#[derive(Clone, Copy)]
pub enum Command {
    Start,
    Remember,
    Recall,
    Help,
    Exit,
    Pomodoro,
    Localize,
    Todo,
    Unknown,
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        all_commands()
            .iter()
            .find(|cmd| cmd.name == input)
            .map(|cmd| cmd.command)
            .unwrap_or(Command::Unknown)
    }
}

pub struct CommandInfo {
    pub name: &'static str,
    pub command: Command,
    pub description: &'static str,
}

pub fn all_commands() -> Vec<CommandInfo> {
    vec![
        CommandInfo { name: "/start",    command: Command::Start,    description: "Démarrer une nouvelle tâche" },
        CommandInfo { name: "/remember", command: Command::Remember, description: "Mémoriser une information" },
        CommandInfo { name: "/recall",   command: Command::Recall,   description: "Afficher ce que j'ai mémorisé" },
        CommandInfo { name: "/help",     command: Command::Help,     description: "Afficher cette aide" },
        CommandInfo { name: "/exit",     command: Command::Exit,     description: "Quitter le programme" },
        CommandInfo { name: "/pomodoro", command: Command::Pomodoro, description: "Lancer un minuteur Pomodoro" },
        CommandInfo { name: "/localize", command: Command::Localize, description: "Me localiser dans le monde" },
        CommandInfo { name: "/todo",     command: Command::Todo,     description: "Gérer une liste de tâches" },
    ]
}



pub fn start() {
    println!("Commençons ! Que veux-tu que je fasse ?");
}

pub fn recall(memory: &str) {
    if memory.is_empty() {
        println!("Je n'ai rien mémorisé.");
    } else {
        println!("Tu m'as demandé de me souvenir de : {}", memory);
    }
}

pub fn help() {
    println!("📘 Commandes disponibles :");
    for cmd in all_commands() {
        println!("{:<12}→ {}", cmd.name, cmd.description);
    }
}

pub fn pomodoro() {
    let pomodoro = 5;
    let pomodoro_pause = 5;
    let mut temps = 0;

    println!("🍅 Pomodoro démarre: ");

    for i in 1..=4 {
        println!("Cycle numéro {}", i);

        while temps < pomodoro {
            temps += 1;
            let m = (pomodoro - temps) / 60;
            let s = (pomodoro - temps) % 60;
            print!("\r💼 Travail : {:02}:{:02}", m, s);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        println!("\n⏰ Pause !");
        temps = 0;

        while temps < pomodoro_pause {
            temps += 1;
            let m = (pomodoro_pause - temps) / 60;
            let s = (pomodoro_pause - temps) % 60;
            print!("\r⏸️ Pause : {:02}:{:02}", m, s);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        println!("\n⏰ Fin de la pause !");
        temps = 0;
    }
}

#[derive(Deserialize, Debug)]
struct Location {
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: Option<String>, // latitude,longitude
    ip: Option<String>,
    org: Option<String>,
}

pub fn localize() {
    println!("🔍 Localisation en cours...");

    let response = get("https://ipinfo.io/json");
    match response {
        Ok(resp) => {
            let location: Result<Location, _> = resp.json();
            match location {
                Ok(loc) => {
                    println!("🌍 IP : {}", loc.ip.unwrap_or("Inconnue".to_string()));
                    println!("📍 Ville : {}", loc.city.unwrap_or("Inconnue".to_string()));
                    println!("🗺️ Région : {}", loc.region.unwrap_or("Inconnue".to_string()));
                    println!("🇺🇳 Pays : {}", loc.country.unwrap_or("Inconnu".to_string()));
                    println!("🛰️ Coordonnées : {}", loc.loc.unwrap_or("Inconnues".to_string()));
                    println!("🏢 Fournisseur : {}", loc.org.unwrap_or("Inconnu".to_string()));
                }
                Err(_) => println!("Erreur de lecture des données JSON."),
            }
        }
        Err(_) => println!("Erreur de connexion à l’API."),
    }
}