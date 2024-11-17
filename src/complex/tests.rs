use super::Complex;

#[test]
fn test_new() {
    let z = Complex::new(1, 2);
    assert!(z.x == 1);
    assert!(z.y == 2);
}

#[test]
fn test_equal() {
    assert!(Complex::new(2, 3) == Complex::new(2, 3));
    assert!(Complex::new(2, 3) != Complex::new(2, 4));
    assert!(Complex::new(1, 3) != Complex::new(2, 3));
}

#[test]
fn test_add() {
    assert!(Complex::new(1, 2) + Complex::new(2, 3) == Complex::new(3, 5));
}

#[test]
fn test_add_assign() {
    let mut z = Complex::new(1, 2);
    z += Complex::new(2, 3);
    assert!(z == Complex::new(3, 5));
}

#[test]
fn test_mul() {
    assert!(Complex::new(1, 2) * Complex::new(3, 4) == Complex::new(-5, 10));
}

#[test]
fn test_mul_assign() {
    let mut z = Complex::new(1, 2);    
    z *= Complex::new(3, 4);
    assert!(z == Complex::new(-5, 10));
}
