// Voici mon code

fn pgcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a  
    } else if a > b {
        pgcd(a - b, b)
    } else {
        pgcd(a, b - a)
    }
}
// RESULTAT
fn main() {
    let a = 10;
    let b = 0;
    println!("PGCD({}, {}) = {}", a, b, pgcd(a, b));
}
