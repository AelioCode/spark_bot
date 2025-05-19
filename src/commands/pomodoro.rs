// commands/pomodoro.rs

///Lancer un minuteur Pomodoro

use std::io::{self, Write};

// fonction handle
pub fn handle_pomodoro() {
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