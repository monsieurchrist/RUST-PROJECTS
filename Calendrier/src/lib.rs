/// Vérifie si une date est valide.
pub fn check_date(jour: u32, mois: u32, annee: i32) -> bool {
    // Vérifie que le mois est entre 1 et 12.
    if mois < 1 || mois > 12 {
        return false;
    }

    // Détermine le nombre maximal de jours dans le mois.
    let max_jours = match mois {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, // Mois avec 31 jours.
        4 | 6 | 9 | 11 => 30,             // Mois avec 30 jours.
        2 => {
            if is_leap_year(annee) {
                29 // Février a 29 jours en année bissextile.
            } else {
                28 // Sinon, février a 28 jours.
            }
        }
        _ => unreachable!(), // Ce cas ne devrait jamais se produire.
    };

    // Vérifie que le jour est valide.
    if jour < 1 || jour > max_jours {
        return false;
    }

    true
}

/// Ajoute un certain nombre de jours à une date donnée.
pub fn add_date(jour: u32, mois: u32, annee: i32, jours_a_ajouter: u32) -> Option<(u32, u32, i32)> {
    if !check_date(jour, mois, annee) {
        return None; // La date initiale est invalide.
    }

    let mut total_jours = jour as i32 + jours_a_ajouter as i32;

    // Calcule le nombre de jours dans chaque mois.
    let mut current_mois = mois;
    let mut current_annee = annee;

    loop {
        let jours_dans_le_mois = match current_mois {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap_year(current_annee) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };

        if total_jours <= jours_dans_le_mois {
            break; // On a trouvé le mois final.
        }

        total_jours -= jours_dans_le_mois as i32;

        // Passe au mois suivant.
        current_mois += 1;
        if current_mois > 12 {
            current_mois = 1;
            current_annee += 1;
        }
    }

    Some((total_jours as u32, current_mois, current_annee))
}

/// Retourne le nombre de jours écoulés depuis le 1 janvier 2000.
pub fn jour_date(jour: u32, mois: u32, annee: i32) -> Option<u32> {
    if !check_date(jour, mois, annee) || annee < 2000 {
        return None; // La date est invalide ou antérieure au 1/1/2000.
    }

    let mut jours_ecoules = 0;

    // Ajoute les années complètes depuis 2000 jusqu'à l'année précédente.
    for year in 2000..annee {
        jours_ecoules += if is_leap_year(year) { 366 } else { 365 };
    }

    // Ajoute les mois complets dans l'année actuelle.
    for m in 1..mois {
        jours_ecoules += match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap_year(annee) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };
    }

    // Ajoute les jours du mois actuel.
    jours_ecoules += jour as u32;

    Some(jours_ecoules)
}

/// Calcule la différence en jours entre deux dates.
pub fn days_between(
    jour1: u32,
    mois1: u32,
    annee1: i32,
    jour2: u32,
    mois2: u32,
    annee2: i32,
) -> Option<i32> {
    let jours1 = jour_date(jour1, mois1, annee1)?;
    let jours2 = jour_date(jour2, mois2, annee2)?;

    Some(jours2 as i32 - jours1 as i32)
}

/// Vérifie si une année est bissextile.
fn is_leap_year(annee: i32) -> bool {
    if annee % 400 == 0 {
        true
    } else if annee % 100 == 0 {
        false
    } else if annee % 4 == 0 {
        true
    } else {
        false
    }
}