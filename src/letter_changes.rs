/// Changes letter to next ascii letter (Wrap around z/Z to a/A)
/// and capitalizes vowels.
/// # Examples
/// ```
/// let input = "hello*3";
/// assert_eq!(&coderbyte_rust::letter_changes(input), "Ifmmp*3");
/// ```
pub fn letter_changes(s: &str) -> String {
    s.chars()
        .map(|c| ascii_vowel_to_uppercase(increment_ascii_alphabetic(c)))
        .collect()
}

/// Increment an ascii character to the next character e.g. 'a' to 'b'.
/// Wraps around 'z'/'Z' to 'a'/'A'.
fn increment_ascii_alphabetic(c: char) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }
    match c {
        'z' => 'a',
        'Z' => 'A',
        _ => (c as u8 + 1) as char,
    }
}

/// Turns english vowels to uppercase
fn ascii_vowel_to_uppercase(c: char) -> char {
    if "aeiou".contains(c) {
        return c.to_ascii_uppercase();
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_short_string() {
        let input = "Hello";
        let expected = "Ifmmp";
        let result = letter_changes(input);
        assert_eq!(&result, expected);
    }

    #[test]
    fn longer_string() {
        let input = "This is sparta!";
        let expected = "UIjt jt tqbsUb!";
        let result = letter_changes(input);
        assert_eq!(&result, expected);
    }
}
