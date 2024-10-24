/// Reverse a string
/// # Examples
/// ```
/// let input = "Hello";
/// assert_eq!(&coderbyte_rust::first_reverse(input), "olleH");
/// ```
pub fn first_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_short_string() {
        let input = "Hello world";
        let expected = "dlrow olleH";
        let result = first_reverse(input);
        assert_eq!(&result, expected);
    }

    #[test]
    fn longer_string() {
        let input = "Hi, this is a long string";
        let expected = "gnirts gnol a si siht ,iH";
        let result = first_reverse(input);
        assert_eq!(&result, expected);
    }
}
