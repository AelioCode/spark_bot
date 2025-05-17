use std::thread;
use std::time::Duration;
use crate::core::context::Context;

fn parse_duration(duree: &str) -> Option<Duration> {
    let duree = duree.trim().to_lowercase();

    // Cas : "10min" ou "10 minutes"
    let (nombre, unite) = if duree.contains(' ') {
        let parts: Vec<&str> = duree.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        (parts[0], parts[1])
    } else {
        // Séparation manuelle entre les chiffres et les lettres
        let digits_end = duree
            .chars()
            .position(|c| !c.is_digit(10))
            .unwrap_or(duree.len());

        let (n, u) = duree.split_at(digits_end);
        (n, u)
    };

    // Parsing et conversion en Duration
    if let Ok(valeur) = nombre.parse::<u64>() {
        match unite {
            "s" | "seconde" | "secondes" => Some(Duration::from_secs(valeur)),
            "m" | "min" | "minute" | "minutes" => Some(Duration::from_secs(valeur * 60)),
            "h" | "heure" | "heures" => Some(Duration::from_secs(valeur * 3600)),
            "j" | "jour" | "jours" => Some(Duration::from_secs(valeur * 86400)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn handle_remind(ctx: &mut Context, input: &str) {
    if let Some(trimmed) = input.strip_prefix("/remind ") {
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() == 2 {
            let message = parts[0].trim();
            let duration_str = parts[1].trim();
            println!("⏰ Tu veux que je te rappelle : \"{}\" dans {}", message, duration_str);

            if let Some(duration) = parse_duration(duration_str) {
                let message = message.to_string();
                thread::spawn(move || {
                    thread::sleep(duration);
                    println!("🔔 Rappel : {}", message);
                });
            } else {
                println!("❌ Durée invalide. Essaie avec un format comme '10min' ou '1 heure'.");
            }
        } else {
            println!("❌ Format incorrect. Exemple : /remind boire de l’eau, 10min");
        }
    }
}
