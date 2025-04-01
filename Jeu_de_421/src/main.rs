use rand::Rng; // Importe la bibliothèque `rand` pour générer des nombres aléatoires.
use rand::seq::SliceRandom; // Pour utiliser shuffle.
use std::io; // Pour interagir avec l'utilisateur via l'entrée/sortie standard.
use std::collections::HashMap; // Pour gérer les scores des joueurs.
use std::thread; // Pour ajouter des délais dans le programme.
use std::time::Duration; // Pour définir la durée des délais.

fn main() {
    println!("Bienvenue au Squid Game !");
    println!("Veuillez patienter... Début du jeu...");
    thread::sleep(Duration::from_secs(2));

    loop {
        let winner = play_full_game();

        // Afficher le gagnant final.
        println!(
            "\nLe gagnant final est {} ! Il remporte 2 Milliards d'Euros !",
            winner
        );

        // Demander si l'utilisateur veut rejouer.
        println!("\nVoulez-vous rejouer ? (Oui/Non) : ");
        let mut replay_input = String::new();
        io::stdin().read_line(&mut replay_input).expect("Erreur lors de la lecture de l'entrée.");
        if !replay_input.trim().to_lowercase().starts_with('o') {
            println!("Merci d'avoir joué ! Au revoir !");
            break;
        }
    }
}

/// Fonction principale qui simule une partie complète (3 manches).
fn play_full_game() -> String {
    // Liste initiale des joueurs.
    let players = vec![
        "Loup".to_string(),
        "Agneau".to_string(),
        "Tigre".to_string(),
        "Chien".to_string(),
        "Poisson".to_string(),
        "Balance".to_string(),
        "Gorille".to_string(),
        "Corbeau".to_string(),
    ];

    // Manche 1 : Jeu de 421.
    println!("\n=== MANCHE 1 : JEU DE 421 ===");
    let remaining_players = play_421(&players);

    // Manche 2 : Pierre-Papier-Ciseaux.
    println!("\n=== MANCHE 2 : PIERRE-PAPIER-CISEAUX ===");
    let final_player = play_pierre_papier_ciseaux(&players); // Tous les joueurs participent.

    // Manche 3 : Joker pour les joueurs éliminés.
    println!("\n=== MANCHE 3 : JOKER ===");
    let eliminated_players: Vec<String> = players
        .iter()
        .filter(|p| !remaining_players.contains(p)) // Correction ici.
        .cloned()
        .collect();
    let ultimate_winner = play_joker(&eliminated_players);

    ultimate_winner
}

/// Simule la manche de 421 avec animations.
fn play_421(players: &[String]) -> Vec<String> {
    let mut scores: HashMap<String, u8> = players.iter().map(|p| (p.clone(), 0)).collect();
    let mut round = 1;

    println!("Début de la manche de 421 !");
    while scores.len() > 1 && !scores.values().any(|&score| score >= 21) {
        println!("\n--- Début du Tour {} ---", round);
        thread::sleep(Duration::from_secs(1));

        // Chaque joueur joue son tour.
        for player in scores.keys().cloned().collect::<Vec<String>>() {
            let roll = roll_dice();
            let points = calculate_points(&roll);
            scores.entry(player.clone()).and_modify(|s| *s += points);

            println!(
                "{} lance les dés... {} ! ({} points), score total: {}",
                player,
                format_dice(&roll),
                points,
                scores[&player]
            );
            thread::sleep(Duration::from_secs(1));

            // Vérifier si le joueur a atteint 21 points.
            if scores[&player] >= 21 {
                println!("{} a atteint 21 points !", player);
                return vec![player];
            }
        }

        // Éliminer le joueur avec le score le plus bas.
        if scores.len() > 1 {
            let min_score_player = scores
                .iter()
                .min_by_key(|(_, &score)| score)
                .map(|(player, _)| player.clone())
                .unwrap();
            println!(
                "--- Fin du Tour {} ---\n{} est éliminé avec un score de {}.",
                round,
                min_score_player,
                scores[&min_score_player]
            );
            scores.remove(&min_score_player);
            thread::sleep(Duration::from_secs(2));
        }

        round += 1;
    }

    scores.into_keys().collect()
}

/// Simule la manche de Pierre-Papier-Ciseaux avec animations.
fn play_pierre_papier_ciseaux(players: &[String]) -> String {
    let mut remaining_players = players.to_vec();

    println!("Début de la manche de Pierre-Papier-Ciseaux !");
    while remaining_players.len() > 1 {
        // Créer des duels aléatoires.
        let mut rng = rand::thread_rng();
        remaining_players.shuffle(&mut rng);

        let mut new_round = Vec::new();
        for i in (0..remaining_players.len()).step_by(2) {
            if i + 1 < remaining_players.len() {
                let (winner, loser) = simulate_duel(&remaining_players[i], &remaining_players[i + 1]);
                println!("{} bat {} !", winner, loser);
                new_round.push(winner.to_string());
            } else {
                new_round.push(remaining_players[i].to_string());
            }
        }

        remaining_players = new_round;
        thread::sleep(Duration::from_secs(2));
    }

    remaining_players[0].clone()
}

/// Simule un duel de Pierre-Papier-Ciseaux avec animation.
fn simulate_duel<'a>(player1: &'a str, player2: &'a str) -> (&'a str, &'a str) {
    let mut rng = rand::thread_rng();
    let choices = ["pierre", "papier", "ciseaux"];
    let choice1 = rng.gen_range(0..3);
    let choice2 = rng.gen_range(0..3);

    println!(
        "{} joue contre {}. Prêt... Combat !",
        player1, player2
    );
    thread::sleep(Duration::from_secs(1));

    println!(
        "{} joue {}, {} joue {}.",
        player1, choices[choice1], player2, choices[choice2]
    );
    thread::sleep(Duration::from_secs(1));

    match (choice1, choice2) {
        (0, 1) | (1, 2) | (2, 0) => {
            println!("Victoire de {} !", player2);
            (player2, player1)
        }
        (0, 2) | (1, 0) | (2, 1) => {
            println!("Victoire de {} !", player1);
            (player1, player2)
        }
        _ => {
            if rng.gen_bool(0.5) {
                println!("Victoire de {} !", player1);
                (player1, player2)
            } else {
                println!("Victoire de {} !", player2);
                (player2, player1)
            }
        }
    }
}

/// Simule la manche Joker pour les joueurs éliminés.
fn play_joker(eliminated_players: &[String]) -> String {
    println!("Bienvenue dans la manche Joker !");
    println!("Les joueurs éliminés doivent choisir un chiffre entre 1 et 5.");

    let mut ultimate_winner = "Personne".to_string();
    for player in eliminated_players {
        let secret_number = rand::thread_rng().gen_range(1..=5);
        println!("\n{} doit choisir un chiffre...", player);

        println!("Devinez le chiffre de {} (entre 1 et 5) :", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée.");
        let guess: u8 = input.trim().parse().unwrap_or(0);

        if guess == secret_number {
            println!(
                "Vous avez deviné correctement ! {} peut rentrer chez lui.",
                player
            );
            ultimate_winner = player.clone();
        } else {
            println!("Mauvaise réponse. {} meurt.", player);
        }
        thread::sleep(Duration::from_secs(2));
    }

    ultimate_winner
}

/// Simule le lancer de 3 dés et retourne un tableau contenant les résultats.
fn roll_dice() -> [u8; 3] {
    let mut rng = rand::thread_rng();
    [rng.gen_range(1..=6), rng.gen_range(1..=6), rng.gen_range(1..=6)]
}

/// Calcule les points en fonction de la combinaison obtenue.
fn calculate_points(dice: &[u8; 3]) -> u8 {
    let mut counts = [0; 7];
    for &value in dice {
        counts[value as usize] += 1;
    }

    if counts[4] == 1 && counts[2] == 1 && counts[1] == 1 {
        10 // Combinaison 4_2_1.
    } else if counts[1] == 3 {
        7 // Trois as.
    } else if counts[1] == 2 && counts[6] == 1 {
        6 // Deux as + 6.
    } else if counts[6] == 3 {
        6 // Trois six.
    } else if counts[1] == 2 && counts[5] == 1 {
        5 // Deux as + 5.
    } else if counts[5] == 3 {
        5 // Trois cinq.
    } else if counts[1] == 2 && counts[4] == 1 {
        4 // Deux as + 4.
    } else if counts[4] == 3 {
        4 // Trois quatre.
    } else if counts[1] == 2 && counts[3] == 1 {
        3 // Deux as + 3.
    } else if counts[3] == 3 {
        3 // Trois trois.
    } else if counts[1] == 2 && counts[2] == 1 {
        2 // Deux as + 2.
    } else if counts[2] == 3 {
        2 // Trois deux.
    } else if counts[3] == 1 && counts[2] == 1 && counts[1] == 1 {
        1 // Combinaison 3+2+as.
    } else {
        1 // Autres combinaisons.
    }
}

/// Formate les résultats des dés pour une meilleure lisibilité.
fn format_dice(dice: &[u8; 3]) -> String {
    dice.iter().map(|&d| d.to_string()).collect::<Vec<_>>().join(", ")
}






# Activez mod_rewrite
RewriteEngine On

# Redirige /onlyoffice (sans slash) vers /onlyoffice/ (avec slash)
RewriteCond %{REQUEST_URI} ^/onlyoffice/?$
RewriteRule ^/onlyoffice/?$ /onlyoffice/welcome/ [R=301,L]
#RedirectMatch ^/onlyoffice$ /onlyoffice/

# Pour toutes les requêtes commençant par /onlyoffice/ :
# La règle retire le préfixe et transmet la suite au backend OnlyOffice.
ProxyPass /onlyoffice/ http://127.0.0.1:8082/
ProxyPassReverse /onlyoffice/ http://127.0.0.1:8082/

# Pour les WebSocket de OnlyOffice
#RewriteRule ^/onlyoffice/websocket/(.*)$ ws://127.0.0.1:8082/websocket/$1 [P,L]
ProxyPass        /onlyoffice/websocket/ ws://127.0.0.1:8082/websocket/
ProxyPassReverse /onlyoffice/websocket/ ws://127.0.0.1:8082/websocket/