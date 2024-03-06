fn fizzbuzz(number: i8) -> String {
    if number == 3 {
        return "fizz".to_string();
    }
    return number.to_string();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fizzbuzz_1_returns_1() {
        assert_eq!(fizzbuzz(1i8), "1");
    }

    #[test]
    fn test_fizzbuzz_2_returns_2() {
        assert_eq!(fizzbuzz(2i8), "2");
    }

    #[test]
    fn test_fizzbuzz_3_returns_3() {
        assert_eq!(fizzbuzz(3i8), "fizz");
    }
}