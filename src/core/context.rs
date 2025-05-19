// core/context.rs

use std::collections::HashMap;
use crate::core::structures::Reminder;

// Mémoire de Spark
pub struct Context {
    pub memory: String, // pour remember/recall
    pub todo_list: HashMap<String, Vec<String>>, // pour /todo
    pub reminders: Vec<Reminder>, // pour /remind
}
