/// Doubles a number.
pub fn double(x: u32) -> u32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::double;

    #[fixture]
    fn two() -> u32 {
        2
    }

    #[rstest]
    fn doubles_fixture(two: u32) {
        let doubled = double(two);
        assert_eq!(doubled, 4);
    }

    #[rstest]
    #[case(1, 2)]
    #[case(2, 4)]
    fn doubles_cases(#[case] input: u32, #[case] expected: u32) {
        assert_eq!(double(input), expected);
    }
}
