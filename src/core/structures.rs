// core/structure.rs

/// Centralise les structures nécessaires pour le bot et ses commandes

// Structure pour la commande /remind
pub struct Reminder {
    pub message: String,
    pub duration: std::time::Duration,
}