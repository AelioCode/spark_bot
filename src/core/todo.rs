use std::collections::HashMap;


use crate::core::input::get_input;


fn todo(todo_list: &mut HashMap<String, Vec<String>>) {
    println!("📒 Gestionnaire de listes");

    if todo_list.is_empty() {
        println!("Il n'y a pas de liste")
    }
    else {
            println!("Voici les listes existantes :");
    for key in todo_list.keys() {
        println!("- {}", key);
        }
    }

    loop {
        let input = get_input();

        match input.as_str() {
            "/new" => {
                println!("Nom de la nouvelle liste :");
                let nom = get_input();
                todo_list.insert(nom.clone(), Vec::new());
                println!("✅ Liste '{}' créée.", nom);
            }

            "/show" => {
                println!("Quelle liste veux-tu afficher ?");
                let nom = get_input();
                match todo_list.get(&nom) {
                    Some(elements) => {
                        if elements.is_empty() {
                            println!("📭 La liste est vide.");
                        } else {
                            println!("📋 Contenu de la liste '{}':", nom);
                            for e in elements {
                                println!("- {}", e);
                            }
                        }
                    }
                    None => println!("❌ Liste introuvable."),
                }
            }

            "/edit" => {
                println!("Quelle liste veux-tu éditer ?");
                let nom = get_input();

                if let Some(list) = todo_list.get_mut(&nom) {
                    loop {
                        println!("(édition de '{}') Tape /add, /remove, /show ou /exit :", nom);
                        let cmd = get_input();

                        match cmd.as_str() {
                            "/add" => {
                                println!("Nom du nouvel élément :");
                                let element_to_add = get_input();
                                list.push(element_to_add);
                                println!("✅ Ajouté");
                            }
                            "/remove" => {
                                println!("Nom de l'élément à supprimer :");
                                let element_to_delete = get_input();
                                let before_len = list.len();
                                list.retain(|e| e != &element_to_delete);
                                if list.len() < before_len {
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

            "/exit" => {
                println!("À bientôt !");
                break;
            }

            _ => println!("Commande inconnue !"),
        }
    }
}