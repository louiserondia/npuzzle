#[cfg(test)]
mod invalid {
    use super::super::parse;

    #[test]
    fn duplicated_value() {
        let res = parse(include_str!("test_inputs/invalid/duplicated_value.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn grid_size_1() {
        let res = parse(include_str!("test_inputs/invalid/grid_size_1.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn grid_size_2() {
        let res = parse(include_str!("test_inputs/invalid/grid_size_2.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn lines_count() {
        let res = parse(include_str!("test_inputs/invalid/lines_count.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn negative_size() {
        let res = parse(include_str!("test_inputs/invalid/negative_size.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn negative_value() {
        let res = parse(include_str!("test_inputs/invalid/negative_value.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn skipped_value() {
        let res = parse(include_str!("test_inputs/invalid/skipped_value.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn wrong_value() {
        let res = parse(include_str!("test_inputs/invalid/wrong_value.txt"));
        assert!(res.is_err());
    }

    #[test]
    fn zero_size() {
        let res = parse(include_str!("test_inputs/invalid/zero_size.txt"));
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod valid {
    use super::super::parse;
    use crate::grid::Grid;

    #[test]
    fn big_snail() {
        let res = parse(include_str!("test_inputs/valid/big_snail.txt"));
        assert!(res.is_ok());
        let g = res.unwrap();
        assert!(g.size == 10);
        assert!(g.v == Grid::create_solved_grid(10).v);
    }

    #[test]
    fn comments() {
        let res = parse(include_str!("test_inputs/valid/comments.txt"));
        assert!(res.is_ok());
        let g = res.unwrap();
        assert!(g.size == 3);
        assert!(g.v == vec![3, 2, 6, 1, 4, 0, 8, 7, 5]);
    }

    #[test]
    fn whitespaces() {
        let res = parse(include_str!("test_inputs/valid/whitespaces.txt"));
        assert!(res.is_ok());
        let g = res.unwrap();
        assert!(g.size == 3);
        assert!(g.v == vec![3, 2, 6, 1, 4, 0, 8, 7, 5]);
    }
}
