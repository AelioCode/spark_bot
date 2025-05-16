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


# Liste des tâches

- [x] /start : message de bienvenue et configuration initiale
- [x] /help : liste des commandes disponibles
- [x] /remember : Spark retient une info pour plus tard (ex : /remember mon code vwifi est 1234)
- [x] /recall : retrouve une info mémorisée
- [x] /pomodoro
- [x] /localize : détecte ton emplacement (via IP ou config) et te donne des infos locales
- [x] /todo : gérer une liste de tâches (ajouter, supprimer, lister)

- [ ] /log : Journalisation des actions (log des tâches effectuées)

- [ ] /prix : Donne le prix actuel d'une crypto
- [ ] /alert : Fixe une alerte de prix d'une crypto

- [ ] /remind : créer un rappel (ex : /remind 10min boire de l’eau)
- [ ] /note : enregistrer une note rapide
- [ ] /habit : Suivi d’habitudes (habit tracker)
- [ ] /weather : météo actuelle d’une ville
- [ ] /find : Recherche web rapide (Ark AI)
- [ ] /translate : traduction rapide d’un mot ou d’une phrase
- [ ] /download : Téléchargement de vidéo (yt-dlp)
- [ ] /summarize : résumer un texte ou un article
- [ ] /encrypt : chiffrer un message
- [ ] /settings : modifier les préférences de l’utilisateur
- [ ] /send_mail Envoi d’e-mails automatisés
- [ ] /send_msg Envoi de message automatisés
- [ ] /status : état actuel du bot ou de certaines tâches
- [ ] /schedule : afficher ou modifier ton planning
- [ ] /news : dernières actualités (via flux RSS ou API)
- [ ] /define : définition d’un mot (via dictionnaire en ligne)
- [ ] /ask : poser une question à une IA (si tu connectes un LLM)
- [ ] /calc : calculatrice intelligente (ex : /calc (3+2)*5)
- [ ] /convert : conversion d’unités ou de devises
- [ ] /scan : scanner un dossier pour détecter des fichiers
- [ ] /backup : lancer une sauvegarde automatique
- [ ] /clean : nettoyer les fichiers temporaires ou inutiles
- [ ] /upload : envoyer un fichier vers un cloud ou un serveur
- [ ] /quote : citation inspirante aléatoire
- [ ] /joke : blague du jour
- [ ] /vault : stocker des infos sensibles localement (mots de passe, clés API)
- [ ] /open : ouvre un lien, un fichier ou une app selon un alias (ex : /open projet)
- [ ] /report : génère un rapport journalier ou hebdo