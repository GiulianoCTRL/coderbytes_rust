/// Check if number is a perfect square
fn is_perfect_square(x: i64) -> bool {
    let s = (x as f64).sqrt() as i64;
    s * s == x
}

/// Check if a number is of the fibonacci sequence
/// ```
/// assert!(coderbyte_rust::fibonacci_checker(8));
/// ```
pub fn fibonacci_checker(num: i32) -> bool {
    let num = num as i64;
    if num < 0 {
        return false;
    }

    let expr1: i64 = 5 * num * num + 4;
    let expr2: i64 = 5 * num * num - 4;

    if is_perfect_square(expr1) || is_perfect_square(expr2) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_square_correct_evaluation() {
        let nums: [i64; 10] = [1, 4, 9, 16, 25, 36, 49, 64, 81, 100];
        assert!(nums.iter().all(|n| is_perfect_square(*n)));
    }

    #[test]
    fn fibonacci_checker_correct_evaluation() {
        let test_numbers = [0i32, 1, 2, 3, 4, 5, 6, 7, 8, 13, 21, 22, 34, 54, 55, 89, 144, 233, 377, 610, 987, 1000];
        let expected = vec![
            true,  // 0
            true,  // 1
            true,  // 2
            true,  // 3
            false, // 4
            true,  // 5
            false, // 6
            false, // 7
            true,  // 8
            true,  // 13
            true,  // 21
            false, // 22
            true,  // 34
            false, // 54
            true,  // 55
            true,  // 89
            true,  // 144
            true,  // 233
            true,  // 377
            true,  // 610
            true,  // 987
            false, // 1000
        ];
        assert_eq!(
            test_numbers
                .iter()
                .map(|n| fibonacci_checker(*n))
                .collect::<Vec<bool>>(),
            expected
        );
    }
}
