// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    // using product
    // (1..=num).product()

    // using fold
    // (1..=num).fold(1, |acc, x| acc * x)

    // using rfold
    (1..=num).rfold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
