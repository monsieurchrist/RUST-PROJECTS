use std::process::Command;

fn main() {
    // Étape 1 : Lister les fichiers du répertoire courant
    println!("--- Liste des fichiers dans le répertoire courant ---");
    let list_files = Command::new("cmd")
        .arg("/c")
        .arg("dir /B") // List files in current directory
        .output()
        .expect("Échec de l'exécution de 'dir'");

    match String::from_utf8(list_files.stdout) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Erreur lors de la conversion UTF-8 : {}", e),
    }

    // Étape 2 : Rechercher le fichier "secret.conf"
    println!("\n--- Recherche du fichier 'secret.conf' ---");
    let search_secret = Command::new("cmd")
        .arg("/c")
        .arg("dir /S /B secret.conf") // Search for secret.conf recursively
        .output()
        .expect("Échec de l'exécution de 'dir /S /B'");

    let secret_path = match String::from_utf8(search_secret.stdout) {
        Ok(path) => path.trim().to_string(),
        Err(e) => {
            eprintln!("Erreur lors de la conversion UTF-8 : {}", e);
            return;
        }
    };

    if secret_path.is_empty() {
        println!("Le fichier 'secret.conf' n'a pas été trouvé.");
        return;
    }

    println!("Fichier trouvé : {}", secret_path);

    // Étape 3 : Afficher le contenu de "secret.conf"
    println!("\n--- Contenu de 'secret.conf' ---");
    let show_content = Command::new("cmd")
        .arg("/c")
        .arg(format!("type {}", secret_path)) // Ici il affiche le chemin concerné
        .output()
        .expect("Échec de l'exécution de 'type'");

    match String::from_utf8(show_content.stdout) {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Erreur lors de la conversion UTF-8 : {}", e),
    }
}
