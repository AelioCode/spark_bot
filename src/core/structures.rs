// core/structure.rs

/// Centralise les structures et contextes nécessaires pour le bot et ses commandes

use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Structure pour la commande /remind
pub struct Reminder {
    pub message: String,
    pub duration: Duration,
    pub creation_time: SystemTime,
}

/// Mémoire de Spark
pub struct Context {
    pub memory: String, // pour remember/recall
    pub todo_list: HashMap<String, Vec<String>>, // pour /todo 
    pub reminders: Arc<Mutex<Vec<Reminder>>>, // pour /remind : stockage thread-safe et clonable pour les handlers qui spawn des threads
}