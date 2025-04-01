// Importe la bibliothèque `rust_date`.
use rust_date::*;
use std::io;

fn main() {
    println!("Bienvenue dans le programme Rust Date !");
    println!("Veuillez entrer une date au format 'jour mois année' (ex: 3 3 2024) :");

    // Lire et valider la première date.
    let (mut jour1, mut mois1, mut annee1) = loop {
        match read_date("Première date") {
            Ok(date) => {
                if check_date(date.0, date.1, date.2) {
                    println!("La date {} {} {} est correcte.", date.0, date.1, date.2);
                    break date; // Sortir de la boucle avec la date valide.
                } else {
                    println!("La date {} {} {} est incorrecte. Veuillez réessayer.", date.0, date.1, date.2);
                }
            }
            Err(err) => println!("{}", err),
        }
    };

    // Demander si l'utilisateur veut comparer avec une autre date.
    if ask_yes_no("Voulez-vous comparer cette date avec une autre ? (Oui/Non)") {
        let (jour2, mois2, annee2) = loop {
            match read_date("Deuxième date") {
                Ok(date) => {
                    if check_date(date.0, date.1, date.2) {
                        println!("La date {} {} {} est correcte.", date.0, date.1, date.2);
                        break date; // Sortir de la boucle avec la date valide.
                    } else {
                        println!("La date {} {} {} est incorrecte. Veuillez réessayer.", date.0, date.1, date.2);
                    }
                }
                Err(err) => println!("{}", err),
            }
        };

        // Calculer le nombre de jours entre les deux dates.
        if let Some(diff) = days_between(jour1, mois1, annee1, jour2, mois2, annee2) {
            println!(
                "Il y a {} jour(s) entre les deux dates.",
                diff.abs()
            );
        } else {
            println!("Impossible de calculer la différence entre les deux dates.");
        }

        // Proposer d'ajouter des jours à l'une des deux dates.
        if ask_yes_no("Voulez-vous ajouter des jours à l'une des deux dates ? (Oui/Non)") {
            println!("À quelle date souhaitez-vous ajouter des jours ?");
            println!("1. Première date : {} {} {}", jour1, mois1, annee1);
            println!("2. Deuxième date : {} {} {}", jour2, mois2, annee2);

            let choix = loop {
                match read_choice("Votre choix (1 ou 2)", 1, 2) {
                    Ok(num) => break num,
                    Err(err) => println!("{}", err),
                }
            };

            let (jour, mois, annee) = if choix == 1 {
                (jour1, mois1, annee1)
            } else {
                (jour2, mois2, annee2)
            };

            let jours_a_ajouter = loop {
                match read_number("Combien de jours souhaitez-vous ajouter ?") {
                    Ok(num) => break num,
                    Err(err) => println!("{}", err),
                }
            };

            if let Some((nouveau_jour, nouveau_mois, nouvelle_annee)) =
                add_date(jour, mois, annee, jours_a_ajouter)
            {
                println!(
                    "Nouvelle date après ajout de {} jours : {} {} {}",
                    jours_a_ajouter, nouveau_jour, nouveau_mois, nouvelle_annee
                );
            } else {
                println!("Impossible d'ajouter les jours à la date donnée.");
            }
        }
    } else {
        // Si pas de comparaison, proposer directement d'ajouter des jours à la première date.
        let jours_a_ajouter = loop {
            match read_number("Combien de jours souhaitez-vous ajouter à la première date ?") {
                Ok(num) => break num,
                Err(err) => println!("{}", err),
            }
        };

        if let Some((nouveau_jour, nouveau_mois, nouvelle_annee)) =
            add_date(jour1, mois1, annee1, jours_a_ajouter)
        {
            println!(
                "Nouvelle date après ajout de {} jours : {} {} {}",
                jours_a_ajouter, nouveau_jour, nouveau_mois, nouvelle_annee
            );
        } else {
            println!("Impossible d'ajouter les jours à la date donnée.");
        }
    }
}

/// Lit une date depuis l'entrée utilisateur.
fn read_date(prompt: &str) -> Result<(u32, u32, i32), String> {
    println!("{} :", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Erreur lors de la lecture de l'entrée.".to_string())?;
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Entrée incorrecte. Veuillez entrer une date au format 'jour mois année'.".to_string());
    }

    let jour: u32 = parts[0].parse().map_err(|_| "Jour invalide.".to_string())?;
    let mois: u32 = parts[1].parse().map_err(|_| "Mois invalide.".to_string())?;
    let annee: i32 = parts[2].parse().map_err(|_| "Année invalide.".to_string())?;

    Ok((jour, mois, annee))
}

/// Demande à l'utilisateur une réponse Oui/Non.
fn ask_yes_no(prompt: &str) -> bool {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        match input.trim().to_lowercase().as_str() {
            "oui" | "o" => return true,
            "non" | "n" => return false,
            _ => println!("Réponse invalide. Veuillez répondre 'Oui' ou 'Non'."),
        }
    }
}

/// Demande à l'utilisateur un choix numérique entre min et max.
fn read_choice(prompt: &str, min: u32, max: u32) -> Result<u32, String> {
    loop {
        println!("{} (entre {} et {})", prompt, min, max);
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| "Erreur lors de la lecture de l'entrée.".to_string())?;
        if let Ok(num) = input.trim().parse::<u32>() {
            if num >= min && num <= max {
                return Ok(num);
            }
        }
        println!("Choix invalide. Veuillez entrer un nombre entre {} et {}.", min, max);
    }
}

/// Demande à l'utilisateur un nombre entier.
fn read_number(prompt: &str) -> Result<u32, String> {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| "Erreur lors de la lecture de l'entrée.".to_string())?;
        if let Ok(num) = input.trim().parse::<u32>() {
            return Ok(num);
        }
        println!("Valeur invalide. Veuillez entrer un nombre entier.");
    }
}