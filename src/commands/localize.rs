// commands/localize.rs

/// Me localiser dans le monde (IP)

use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Location {
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: Option<String>, // latitude,longitude
    ip: Option<String>,
    org: Option<String>,
}

//fonction handle
pub fn handle_localize() {
    println!("🔍 Localisation en cours...");

    let response = get("https://ipinfo.io/json");
    match response {
        Ok(resp) => {
            let location: Result<Location, _> = resp.json();
            match location {
                Ok(loc) => {
                    println!("🌍 IP : {}", loc.ip.unwrap_or("Inconnue".to_string()));
                    println!("📍 Ville : {}", loc.city.unwrap_or("Inconnue".to_string()));
                    println!("🗺️ Région : {}", loc.region.unwrap_or("Inconnue".to_string()));
                    println!("🇺🇳 Pays : {}", loc.country.unwrap_or("Inconnu".to_string()));
                    println!("🛰️ Coordonnées : {}", loc.loc.unwrap_or("Inconnues".to_string()));
                    println!("🏢 Fournisseur : {}", loc.org.unwrap_or("Inconnu".to_string()));
                }
                Err(_) => println!("Erreur de lecture des données JSON."),
            }
        }
        Err(_) => println!("Erreur de connexion à l’API."),
    }
}