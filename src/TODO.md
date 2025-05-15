# Séparation en modules

src/
├── main.rs
├── core/
│   ├── input.rs
│   ├── commands.rs
│   └── todo.rs

# Utiliser un enum pour les commandes

enum Command {
    Start,
    Remember,
    Recall,
    Help,
    Exit,
    Pomodoro,
    Localize,
    Todo,
    Unknown,
}

# Extraire todo dans un module dédié

# Utiliser une structure de contexte (struct Context)

Pour stocker la mémoire, les listes, et éventuellement l’état du bot :

struct Context {
    memory: String,
    todo_list: HashMap<String, Vec<String>>,
}

# Ajouter des tests unitaires

# Améliorer l’UX : menu contextuel
Plutôt que de demander à chaque fois quoi faire, tu pourrais afficher un menu après chaque commande ou proposer des suggestions.

# Externaliser les textes
Tu pourrais stocker les messages (d’accueil, d’aide, etc.) dans un fichier .json ou .toml pour les modifier sans recompiler.