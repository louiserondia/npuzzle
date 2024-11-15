
use super::Complex;

#[test]
fn test_equal() {
    let z1 = Complex::new(2, 3);
    let z2 = Complex::new(2, 3);
    assert!(z1 == z2);
}
