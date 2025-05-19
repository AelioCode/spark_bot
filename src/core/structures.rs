// core/structure.rs

// Structure pour la commande /remind
pub struct Reminder {
    pub message: String,
    pub duration: std::time::Duration,
}