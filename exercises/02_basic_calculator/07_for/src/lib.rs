// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut n1 = n;
    let mut result = 1;
    for _ in 0..n {
        result = n1 * result;
        n1 -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
