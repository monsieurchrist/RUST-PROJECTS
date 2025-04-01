struct Book {
    reference: String, // Référence unique
    title: String,     // Titre du livre
    author: String,    // Auteur du livre
    year: i32,         // Année d'édition
    pages: i32,        // Nombre de pages
}

// Fonction pour trouver l'auteur d'un livre par son titre
fn find_author_by_title(books: &Vec<Book>, title: &str) -> Option<String> {
    for Book in books {
        if Book.title == title {
            return Some(Book.author.clone()); // Retourne l'auteur si le titre correspond
        }
    }
    None // Si aucun livre ne correspond, retourne None
}

// Fonction pour trouver tous les livres d'un auteur donné
fn find_books_by_author(books: &Vec<Book>, author: &str) -> Option<String> {
    for Book in books {
        if Book.author == author {
            return Some(Book.title.clone()); // Retourne le titre si l'auteur correspond
        }
    }
    None // Si aucun auteur ne correspond, retourne None
}

fn main() {
    // Créer une liste de livres
    let books = vec![
        Book {
            reference: String::from("Rcf1"),
            title: String::from("Le rouge et le noir"),
            author: String::from("Stendhal"),
            year: 1830,
            pages: 100,
        },
        Book {
            reference: String::from("Rcf2"),
            title: String::from("Madame Bovary"),
            author: String::from("Gustave Flaubert"),
            year: 1856,
            pages: 150,
        },
        Book {
            reference: String::from("Rcf3"),
            title: String::from("Le comte de Monte-Cristo"),
            author: String::from("Alexandre Dumas"),
            year: 1844,
            pages: 170,
        },
        Book {
            reference: String::from("Rcf4"),
            title: String::from("L'étranger"),
            author: String::from("Albert Camus"),
            year: 1942,
            pages: 95,
        },
        Book {
            reference: String::from("Rcf5"),
            title: String::from("La chartreuse de Parme"),
            author: String::from("Stendhal"),
            year: 1839,
            pages: 130,
        },
    ];

    // Question 2: Implémenter la fonction qui permet de retourner l’auteur d’un livre connu par son titre.
    if let Some (author)= find_author_by_title(&books, "Hulk"){
        println!("Recherche : L'auteur de ' L'étranger' est {}", author)
    }
}
