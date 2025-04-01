use std::io; // Module pour interagir avec l'entrée/sortie standard

fn main() {
    loop { // Boucle infinie pour maintenir le programme en cours d'exécution
        println!("Bienvenue dans la calculatrice !");
        println!("Veuillez choisir une opération : +, -, *, /");

        // Étape 1 : Demander à l'utilisateur de choisir une opération
        let mut operation = String::new(); // Variable pour stocker l'opération choisie
        io::stdin()
            .read_line(&mut operation) // Lire l'entrée utilisateur
            .expect("Échec de la lecture de l'entrée"); // Gérer les erreurs potentielles

        let operation = operation.trim(); // Supprimer les espaces et les retours à la ligne

        // Vérifier si l'opération choisie est valide
        if !["+", "-", "*", "/"].contains(&operation) {
            println!("Opération invalide. Veuillez réessayer.");
            continue; // Revenir au début de la boucle si l'opération est incorrecte
        }

        // Étape 2 : Demander à l'utilisateur de saisir des nombres
        println!("Entrez vos nombres (séparés par des espaces) :");
        let mut numbers_input = String::new(); // Variable pour stocker les nombres entrés
        io::stdin()
            .read_line(&mut numbers_input)
            .expect("Échec de la lecture des nombres");

        // Convertir l'entrée en une liste de nombres
        let numbers: Vec<f64> = numbers_input
            .trim() // Supprimer les espaces inutiles
            .split_whitespace() // Diviser la chaîne en parties séparées par des espaces
            .filter_map(|num| num.parse::<f64>().ok()) // Essayer de convertir chaque partie en nombre flottant
            .collect();

        // Vérifier si au moins deux nombres ont été entrés
        if numbers.len() < 2 {
            println!("Veuillez entrer au moins deux nombres.");
            continue; // Revenir au début de la boucle si pas assez de nombres
        }

        // Étape 3 : Effectuer le calcul en fonction de l'opération choisie
        let result = match operation {
            "+" => add(&numbers),   // Addition
            "-" => subtract(&numbers), // Soustraction
            "*" => multiply(&numbers), // Multiplication
            "/" => divide(&numbers),   // Division
            _ => {
                println!("Opération non reconnue. Veuillez réessayer.");
                continue; // Revenir au début de la boucle si l'opération est incorrecte
            }
        };

        // Afficher le résultat
        println!("Résultat : {}", result);

        // Continuer indéfiniment sans `break`
    }
}

// Fonction pour additionner une liste de nombres
fn add(numbers: &[f64]) -> f64 {
    numbers.iter().sum() // Somme de tous les nombres dans la liste
}

// Fonction pour soustraire une liste de nombres
fn subtract(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0; // Si la liste est vide, retourner 0
    }
    let mut iter = numbers.iter(); // Créer un itérateur sur les nombres
    let first = *iter.next().unwrap(); // Prendre le premier nombre comme base
    first - iter.sum::<f64>() // Soustraire la somme des autres nombres du premier
}

// Fonction pour multiplier une liste de nombres
fn multiply(numbers: &[f64]) -> f64 {
    numbers.iter().product() // Produit de tous les nombres dans la liste
}

// Fonction pour diviser une liste de nombres
fn divide(numbers: &[f64]) -> f64 {
    if numbers.len() < 2 {
        return 0.0; // Si moins de deux nombres, retourner 0
    }
    let mut iter = numbers.iter(); // Créer un itérateur sur les nombres
    let first = *iter.next().unwrap(); // Prendre le premier nombre comme base
    iter.fold(first, |acc, &num| {
        if num == 0.0 {
            println!("Erreur : Division par zéro !");
            0.0 // Retourner 0 si une division par zéro est détectée
        } else {
            acc / num // Diviser le résultat courant par le nombre suivant
        }
    })
}