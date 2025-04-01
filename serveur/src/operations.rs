/// Calcule la surface d'un rectangle.
pub fn rectangle(length: f64, width: f64) -> f64 {
    length * width
}

/// Calcule la surface d'un triangle.
pub fn triangle(base: f64, height: f64) -> f64 {
    0.5 * base * height
}

/// Calcule la surface d'un cercle.
pub fn cercle(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

/// Calcule la surface d'une sphère.
pub fn sphere_surface(radius: f64) -> f64 {
    4.0 * std::f64::consts::PI * radius * radius
}

/// Calcule le volume d'un parallélépipède.
pub fn parallelepiped(length: f64, width: f64, height: f64) -> f64 {
    length * width * height
}

/// Calcule le volume d'une sphère.
pub fn sphere_volume(radius: f64) -> f64 {
    (4.0 / 3.0) * std::f64::consts::PI * radius * radius * radius
}
