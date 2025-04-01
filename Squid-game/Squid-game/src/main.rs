use rand::{Rng, seq::SliceRandom}; // Version compatible
use std::io::{self, Write};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::fs::OpenOptions;

fn main() {
    println!("Bienvenue au Squid Game !");
    println!("Veuillez patienter... Début du jeu...");
    thread::sleep(Duration::from_secs(2));

    loop {
        let winner = play_full_game();

        println!(
            "\nLe gagnant final est {} ! Il remporte 2 Milliards d'Euros !",
            winner
        );

        println!("\nVoulez-vous rejouer ? (Oui/Non) : ");
        let mut replay_input = String::new();
        io::stdin().read_line(&mut replay_input).expect("Erreur lors de la lecture de l'entrée.");
        if !replay_input.trim().to_lowercase().starts_with('o') {
            println!("Merci d'avoir joué ! Au revoir !");
            break;
        }
    }
}

fn play_421(players: &[String]) -> Vec<String> {
    let mut scores: HashMap<String, u8> = players.iter().map(|p| (p.clone(), 0)).collect();
    let mut round = 1;

    println!("Début de la manche de 421 !");
    while scores.len() > 1 && !scores.values().any(|&score| score >= 21) {
        println!("\n--- Début du Tour {} ---", round);
        thread::sleep(Duration::from_secs(1));

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

            if scores[&player] >= 21 {
                println!("{} a atteint 21 points !", player);
                return vec![player];
            }
        }

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

fn play_pierre_papier_ciseaux(players: &[String]) -> String {
    let mut remaining_players = players.to_vec();

    println!("Début de la manche de Pierre-Papier-Ciseaux !");
    while remaining_players.len() > 1 {
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

fn roll_dice() -> [u8; 3] {
    let mut rng = rand::thread_rng();
    [rng.gen_range(1..=6), rng.gen_range(1..=6), rng.gen_range(1..=6)]
}

fn calculate_points(dice: &[u8; 3]) -> u8 {
    let mut counts = [0; 7];
    for &value in dice {
        counts[value as usize] += 1;
    }

    if counts[4] == 1 && counts[2] == 1 && counts[1] == 1 {
        10
    } else if counts[1] == 3 {
        7
    } else if counts[1] == 2 && counts[6] == 1 {
        6
    } else if counts[6] == 3 {
        6
    } else if counts[1] == 2 && counts[5] == 1 {
        5
    } else if counts[5] == 3 {
        5
    } else if counts[1] == 2 && counts[4] == 1 {
        4
    } else if counts[4] == 3 {
        4
    } else if counts[1] == 2 && counts[3] == 1 {
        3
    } else if counts[3] == 3 {
        3
    } else if counts[1] == 2 && counts[2] == 1 {
        2
    } else if counts[2] == 3 {
        2
    } else if counts[3] == 1 && counts[2] == 1 && counts[1] == 1 {
        1
    } else {
        1
    }
}

fn format_dice(dice: &[u8; 3]) -> String {
    dice.iter().map(|&d| d.to_string()).collect::<Vec<_>>().join(", ")
}

fn play_full_game() -> String {
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

    println!("\n=== MANCHE 1 : JEU DE 421 ===");
    let remaining_players = play_421(&players);
    log_manche("421", &remaining_players, &players, 0);

    println!("\n=== MANCHE 2 : PIERRE-PAPIER-CISEAUX ===");
    let final_player = play_pierre_papier_ciseaux(&players);
    log_manche("Pierre-Papier-Ciseaux", &[final_player.clone()], &players, 0);

    println!("\n=== MANCHE 3 : JOKER ===");
    let eliminated_players: Vec<String> = players
        .iter()
        .filter(|p| !remaining_players.contains(p))
        .cloned()
        .collect();
    let ultimate_winner = play_joker(&eliminated_players);
    log_manche("Joker", &[ultimate_winner.clone()], &eliminated_players, 2_000_000_000);

    ultimate_winner
}

fn log_manche(game: &str, winners: &[String], participants: &[String], sum: u64) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("resultat.txt")
        .expect("Erreur lors de l'ouverture du fichier");

    let losers: Vec<String> = participants
        .iter()
        .filter(|p| !winners.contains(p))
        .cloned()
        .collect();

    writeln!(
        file,
        "Jeu: {}\nVainqueur(s): {}\nPerdant(s): {}\nSomme gagnée: {}\n",
        game,
        winners.join(", "),
        losers.join(", "),
        sum
    ).expect("Erreur lors de l'écriture dans le fichier");
}