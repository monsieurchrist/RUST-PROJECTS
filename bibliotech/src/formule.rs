    pub fn square_surface(side: f64) -> f64 {
        assert!(side >=0.0);
        return side * side
        
    }

    pub fn square_perimeter(side : f64) -> f64 {
        assert!(side >=0.0);
        return 4.0 * side
    }

    pub fn rectangle_surface(length: f64, width: f64) -> f64 {
        assert!(length >=0.0);
        assert!(width >=0.0);
        return  length * width
    }
/*
    pub fn rectangle_perimeter(length: f64, width: f64) -> f64 {
        assert!(length >=0.0);
        assert!(width >=0.0);
        let perimeter = 2.0 * (length + width)
    }

    pub fn circle_surface(rayon:f64) -> f64 {
        assert(rayon >=0.0);
        let surface = std::f64::consts::PI * rayon * rayon;
    }

    pub fn circle_perimeter(rayon:f64) -> f64 {
        assert(rayon >=0.0);
        let perimeter = 2.0 * std::f64::consts::PI * rayon
    }

    pub fn cube_surface(side:f64) -> f64 {
        assert(side >=0.0);
        let surface = side * side * side
    }

    pub fn square_volume(side:f64) -> f64 {
        assert(side >=0.0);
        let volume = 6.0 * side * side
    }
*/