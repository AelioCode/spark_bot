use std::collections::HashMap;
use std::io;

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erreur de lecture");
    buffer.trim().to_string()
}

fn welcome_message() {
    println!("Salut, je suis Spark, le bot qui t'aide à effectuer tes tâches. Tape /exit pour quitter.");
}

fn main() {

    use std::io::{self, Write};
    
    // Dictionnaire des commandes
    let mut commands: HashMap<&str, &str> = HashMap::new();
    commands.insert("/start", "Démarrer une nouvelle tâche");
    commands.insert("/remember", "Mémoriser une information");
    commands.insert("/recall", "Afficher ce que j'ai mémorisé");
    commands.insert("/help", "Afficher cette aide");
    commands.insert("/exit", "Quitter le programme");
    commands.insert("/pomodoro", "Lancer un minuteur Pomodoro (25 min travail / 5 min pause)");



    let mut memory = String::new();
    let pomodoro = 5;
    let pomodoro_pause = 5;

    welcome_message();

    loop {
        let input = get_input();

        match input.as_str() {
            "/start" => println!("Commençons à échanger, donne-moi la tâche que tu veux que j'effectue :"),
            "/remember" => {
                println!("Que dois-je me souvenir ?");
                memory = get_input();
                println!("Ok, je m'en souviendrai !");
            }
            "/recall" => {
                if memory.is_empty() {
                    println!("Je n'ai encore rien mémorisé.");
                } else {
                    println!("Tu m'as demandé de me souvenir de : {}", memory);
                }
            }
            "/pomodoro" => {
                io::stdout().flush().unwrap();
                let mut temps = 0;
                println!("🍅 Pomodoro démarre: ");

                for i in 1..5 {
                    println!(" ↻ Cycle numéro {}\n", i);

                    while temps < pomodoro {
                    temps += 1;
                    let minutes = (pomodoro - temps) / 60;
                    let secondes = (pomodoro - temps) % 60;             
                    print!("\r 💼 Temps de travail restant : {:02}:{:02}", minutes, secondes);
                    io::stdout().flush().unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }

                    println!("\n⏰ Temps écoulé ! Prends une pause !");
                    temps = 0;

                    while temps < pomodoro_pause {
                        temps += 1;
                        let minutes = (pomodoro - temps) / 60;
                        let secondes = (pomodoro - temps) % 60;             
                        print!("\r ⏸️ Temps de pause restant : {:02}:{:02}", minutes, secondes);
                        io::stdout().flush().unwrap();
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                    println!("\n⏰ Temps écoulé ! Il faut retourner travailler !");
                    temps = 0;
                }
            }

            "/bip" => {
                //TODO
            }

            "/exit" => {
                println!("À bientôt !");
                break;
            }


            "/help" => {
                println!("Voici les commandes disponibles :");
                for (cmd, desc) in &commands {
                println!("{:<10} → {}", cmd, desc);
                }
            }
            _ => println!("Commande inconnue !"),
        }
    }
}
