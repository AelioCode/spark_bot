// commands/remind.rs

///Créer un rappel pour une certaine durée

use std::thread;
use std::time::Duration;
use std::sync::Arc;
use crate::core::structures::{Context, Reminder};

// Transforme le texte d'entree en durée
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

//fonction handle
pub fn handle_remind(ctx: &mut Context, input: &str) {
    match input {
        "/remind list" => list_reminders(ctx),
        input if input.starts_with("/remind ") => create_reminder(ctx, input),
        _ => println!("❌ Format incorrect. Exemple : /remind boire de l'eau, 10min")
    }
}

fn list_reminders(ctx: &Context) {
    let list = ctx.reminders.lock().unwrap();
    if list.is_empty() {
        println!("📭 Aucun rappel actif");
        return;
    }

    println!("📋 Rappels actifs :");
    for reminder in list.iter() {
        if let Ok(elapsed) = reminder.creation_time.elapsed() {
            if elapsed < reminder.duration {
                let remaining = (reminder.duration - elapsed).as_secs();
                println!("- {} (dans {} secondes)", reminder.message, remaining);
            }
        }
    }
}

fn create_reminder(ctx: &mut Context, input: &str) {
    let trimmed = input.strip_prefix("/remind ").unwrap();
    let parts: Vec<&str> = trimmed.split(',').collect();
    if parts.len() == 2 {
        let message = parts[0].trim().to_string();
        let duration_str = parts[1].trim();

        if let Some(duration) = parse_duration(duration_str) {
            // Créer le reminder
            let reminder = Reminder {
                message: message.clone(),
                duration,
                creation_time: std::time::SystemTime::now(),
            };

            // pousser sous lock dans la liste partagée
            {
                let mut list = ctx.reminders.lock().unwrap();
                list.push(reminder);
            }

            // cloner l'Arc pour le thread qui affichera et nettoiera
            let reminders = Arc::clone(&ctx.reminders);
            let message_clone = message.clone();
            thread::spawn(move || {
                thread::sleep(duration);
                println!("🔔 Rappel : {}", message_clone);
                let mut list = reminders.lock().unwrap();
                list.retain(|r| r.message != message_clone);
            });
        } else {
            println!("❌ Durée invalide. Essaie avec un format comme '10min' ou '1 heure'.");
        }
    } else {
        println!("❌ Format incorrect. Exemple : /remind boire de l'eau, 10min");
    }
}