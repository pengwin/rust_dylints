//! Production code with tests which break the test style.

/// Doubles a number.
pub fn double(x: u32) -> u32 {
    x * 2
}

/// Pair of numbers.
pub struct Pair {
    pub left: u32,
    pub right: u32,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Pair, double};

    fn helper() -> u32 {
        2
    }

    #[test]
    fn plain_test() {
        assert_eq!(double(2), 4);
    }

    #[rstest]
    fn inline_arrange() {
        let pair = Pair { left: 1, right: 2 };
        assert_eq!(double(pair.left), pair.right);
        assert_eq!(helper(), 2);
    }
}
