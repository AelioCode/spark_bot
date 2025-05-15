use std::collections::HashMap;
use std::io::{self, Write};

use reqwest::blocking::get;
use serde::Deserialize;

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
            "/exit" => {
                println!("À bientôt !");
                break;
            }
            "/help" => help(&commands),
            _ => println!("Commande inconnue !"),
        }
    }
}

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erreur de lecture");
    buffer.trim().to_string()
}

pub fn welcome_message() {
    println!("Salut, je suis Spark. Tape /exit pour quitter.");
}

fn start() {
    println!("Commençons ! Que veux-tu que je fasse ?");
}

fn recall(memory: &str) {
    if memory.is_empty() {
        println!("Je n'ai rien mémorisé.");
    } else {
        println!("Tu m'as demandé de me souvenir de : {}", memory);
    }
}

fn help(commands: &HashMap<&str, &str>) {
    println!("Voici les commandes disponibles :");
    for (cmd, desc) in commands {
        println!("{:<10} → {}", cmd, desc);
    }
}

fn pomodoro() {
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

fn localize() {
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
