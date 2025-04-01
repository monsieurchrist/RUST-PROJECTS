
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufRead, BufReader};
use std::thread;
use std::f64::consts::PI;

/// Envoie un message au client avec gestion d'erreur
fn send_response(stream: &mut TcpStream, message: &str) -> Result<(), std::io::Error> {
    // Formatage du message avec retour à la ligne Linux
    stream.write_all(format!("{}\n", message).as_bytes())?;
    stream.flush()
}

/// Effectue les calculs géométriques en fonction de la commande
fn calculate(fonction: &str, params: &str) -> Result<f64, String> {
    // Conversion des virgules en points pour les nombres décimaux
    let params_clean = params.replace(",", ".");
    
    match fonction.trim() {
        // Surface rectangle : longueur * largeur
        "sr" => {
            let vals: Vec<f64> = params_clean
                .split_whitespace()
                .map(|s| s.parse().map_err(|_| format!("Nombre invalide : '{}'", s)))
                .collect::<Result<_, _>>()?;
                
            if vals.len() != 2 {
                return Err("Requiert exactement 2 paramètres".into());
            }
            Ok(vals[0] * vals[1])
        }
        
        // Surface triangle : (base * hauteur) / 2
        "st" => {
            let vals: Vec<f64> = params_clean
                .split_whitespace()
                .map(|s| s.parse().map_err(|_| format!("Nombre invalide : '{}'", s)))
                .collect::<Result<_, _>>()?;
                
            Ok(0.5 * vals[0] * vals[1])
        }
        
        // Surface cercle : π * r²
        "sc" => {
            let r: f64 = params_clean
                .trim()
                .parse()
                .map_err(|_| "Rayon invalide".to_string())?;
            Ok(PI * r.powi(2))
        }
        
        // Surface sphère : 4πr²
        "ss" => {
            let r: f64 = params_clean
                .trim()
                .parse()
                .map_err(|_| "Rayon invalide".to_string())?;
            Ok(4.0 * PI * r.powi(2))
        }
        
        // Volume sphère : (4/3)πr³
        "vs" => {
            let r: f64 = params_clean
                .trim()
                .parse()
                .map_err(|_| "Rayon invalide".to_string())?;
            Ok((4.0 / 3.0) * PI * r.powi(3))
        }
        
        // Volume pavé : long * larg * haut
        "vp" => {
            let vals: Vec<f64> = params_clean
                .split_whitespace()
                .map(|s| s.parse().map_err(|_| format!("Nombre invalide : '{}'", s)))
                .collect::<Result<_, _>>()?;
                
            if vals.len() != 3 {
                return Err("Requiert exactement 3 paramètres".into());
            }
            Ok(vals[0] * vals[1] * vals[2])
        }
        
        // Commande inconnue
        _ => Err("Fonction inconnue".into()),
    }
}

/// Gère la connexion d'un client
fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();

    // Envoi du message de bienvenue initial
    let _ = send_response(&mut stream, "Bienvenue! Entrez une fonction (sr, st, sc, ss, vp, vs) :");

    // Boucle principale de traitement des commandes
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => break, // Client déconnecté
            Ok(_) => {
                let fonction = buffer.trim().to_lowercase();

                // Détermination du prompt en fonction de la commande
                let prompt = match fonction.as_str() {
                    "sr" => "Entrez longueur et largeur (ex: 5 4)",
                    "st" => "Entrez base et hauteur (ex: 3 2.5)",
                    "sc" | "ss" | "vs" => "Entrez le rayon (ex: 2.5)",
                    "vp" => "Entrez longueur, largeur, hauteur (ex: 3 4 5)",
                    "q" => {
                        let _ = send_response(&mut stream, "Au revoir!");
                        break;
                    }
                    _ => {
                        let _ = send_response(&mut stream, "ERREUR: Fonction invalide");
                        continue;
                    }
                };

                // Envoi du prompt et lecture des paramètres
                if let Err(e) = send_response(&mut stream, prompt) {
                    eprintln!("Erreur: {}", e);
                    break;
                }

                buffer.clear();
                if let Err(e) = reader.read_line(&mut buffer) {
                    eprintln!("Erreur: {}", e);
                    break;
                }

                // Calcul et envoi du résultat
                match calculate(&fonction, &buffer.trim()) {
                    Ok(res) => {
                        let _ = send_response(&mut stream, &format!("RESULTAT: {:.2}", res));
                    }
                    Err(e) => {
                        let _ = send_response(&mut stream, &format!("ERREUR: {}", e));
                    }
                }
            }
            Err(e) => {
                eprintln!("Erreur: {}", e);
                break;
            }
        }
    }
    println!("Client déconnecté");
}

/// Ici c'est le Point d'entrée principal du serveur
fn main() {
    // Création du listener TCP
    let listener = TcpListener::bind("addresse_du_serveur:6000").unwrap();
    println!("Serveur en écoute sur port 6000...");

    // Gestion des connexions entrantes
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nouvelle connexion: {:?}", stream.peer_addr());
                // Lancement d'un thread par client
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Erreur: {}", e),
        }
    }
}
