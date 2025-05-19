// commands/todo.rs

///Gérer une liste de tâches
/// /new list
/// /edit list
/// /show list
/// /add item
/// /remove item

use std::collections::HashMap;
use crate::core::input::get_input;

// fonction handle
pub fn handle_todo(todo_list: &mut HashMap<String, Vec<String>>) {
    println!("📒 Gestionnaire de listes");

    afficher_listes(todo_list);

    loop {
        let input = get_input();

        match input.as_str() {
            "/new" => create_list(todo_list), // créer une nouvelle liste
            "/show" => show_list(todo_list), // affiche les éléments d'une liste
            "/edit" => edit_list(todo_list), // rentre dans l'éditeur de liste
            "/exit" => { // sortie du gestionnaire
                println!("Fin du gestionnaire");
                break;
            }
            _ => println!("Commande inconnue !"),
        }
    }
}

// Affiche les listes existantes
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

// Créer une nouvelle liste
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

// Affiche les éléments d'une liste
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

fn remove_list(todo_list: &HashMap<String, Vec<String>>) {
    println!(("Quelle liste veux-tu supprimer ?"));
    let nom = get_input();
    if todo_list.contains_key(&nom) {
        todo_list.remove(&nom);
        println!("✅ Liste '{}' supprimée.", nom);
    } else {
        println!("❗ Cette liste n'existe pas.");

    }
}

// Rentre dans l'éditeur de liste
fn edit_list(todo_list: &mut HashMap<String, Vec<String>>) {
    println!("Quelle liste veux-tu éditer ?");
    let nom = get_input();

    if let Some(list) = todo_list.get_mut(&nom) {
        loop {
            println!("(édition de '{}') Tape /add, /remove, /show ou /exit :", nom);
            match get_input().as_str() {

                // Ajouter un élément à la liste
                "/add" => {
                    println!("Nom du nouvel élément :");
                    let element = get_input();
                    list.push(element);
                    println!("✅ Ajouté");
                }

                // Supprime un élément de la liste
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

                // Affiche les éléments de la liste
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

                // Sortie de l'éditeur
                "/exit" => break,
                _ => println!("Commande inconnue."),
            }
        }
    } else {
        println!("❌ Liste introuvable.");
    }
}