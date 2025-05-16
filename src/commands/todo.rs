use std::collections::HashMap;
use crate::core::input::get_input;

pub fn handle_todo(todo_list: &mut HashMap<String, Vec<String>>) {
    println!("📒 Gestionnaire de listes");

    afficher_listes(todo_list);

    loop {
        let input = get_input();

        match input.as_str() {
            "/new" => create_list(todo_list),
            "/show" => show_list(todo_list),
            "/edit" => edit_list(todo_list),
            "/exit" => {
                println!("À bientôt !");
                break;
            }
            _ => println!("Commande inconnue !"),
        }
    }
}

fn afficher_listes(todo_list: &HashMap<String, Vec<String>>) {
    if todo_list.is_empty() {
        println!("Il n'y a pas de liste.");
    } else {
        println!("Voici les listes existantes :");
        for key in todo_list.keys() {
            println!("- {}", key);
        }
    }
}

fn create_list(todo_list: &mut HashMap<String, Vec<String>>) {
    println!("Nom de la nouvelle liste :");
    let nom = get_input();
    if todo_list.contains_key(&nom) {
        println!("❗ Une liste avec ce nom existe déjà.");
    } else {
        todo_list.insert(nom.clone(), Vec::new());
        println!("✅ Liste '{}' créée.", nom);
    }
}

fn show_list(todo_list: &HashMap<String, Vec<String>>) {
    println!("Quelle liste veux-tu afficher ?");
    let nom = get_input();
    match todo_list.get(&nom) {
        Some(elements) if elements.is_empty() => println!("📭 La liste est vide."),
        Some(elements) => {
            println!("📋 Contenu de la liste '{}':", nom);
            for e in elements {
                println!("- {}", e);
            }
        }
        None => println!("❌ Liste introuvable."),
    }
}

fn edit_list(todo_list: &mut HashMap<String, Vec<String>>) {
    println!("Quelle liste veux-tu éditer ?");
    let nom = get_input();

    if let Some(list) = todo_list.get_mut(&nom) {
        loop {
            println!("(édition de '{}') Tape /add, /remove, /show ou /exit :", nom);
            match get_input().as_str() {
                "/add" => {
                    println!("Nom du nouvel élément :");
                    let element = get_input();
                    list.push(element);
                    println!("✅ Ajouté");
                }
                "/remove" => {
                    println!("Nom de l'élément à supprimer :");
                    let element = get_input();
                    let len_avant = list.len();
                    list.retain(|e| e != &element);
                    if list.len() < len_avant {
                        println!("✅ Supprimé");
                    } else {
                        println!("❌ Élément non trouvé.");
                    }
                }
                "/show" => {
                    if list.is_empty() {
                        println!("📭 La liste est vide.");
                    } else {
                        println!("📋 Contenu de la liste '{}':", nom);
                        for e in list.iter() {
                            println!("- {}", e);
                        }
                    }
                }
                "/exit" => break,
                _ => println!("Commande inconnue."),
            }
        }
    } else {
        println!("❌ Liste introuvable.");
    }
}