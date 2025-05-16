pub fn handle_recall(memory: &str) {
    if memory.is_empty() {
        println!("Je n'ai rien mémorisé.");
    } else {
        println!("Tu m'as demandé de me souvenir de : {}", memory);
    }
}