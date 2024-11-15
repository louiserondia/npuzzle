use super::parse;

#[test]
fn test1() {
    let res = parse(include_str!("inputs/1.txt"));
    assert!(res.is_ok());
}

#[test]
fn test2() {
    let res = parse(include_str!("inputs/2.txt"));
    assert!(res.is_err());
}
