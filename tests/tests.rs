fn fizzbuzz(number: i8) -> String {
    let mul3 = number % 3 == 0;
    let mul5 = number % 5 == 0;
    let is_buzz = mul5;
    let is_fizz = mul3;
    let is_fizz_buzz = mul3 && mul5;

    if is_fizz_buzz {
        return "fizzbuzz".to_string();
    }

    if is_fizz {
        return "fizz".to_string();
    }

    if is_buzz {
        return "buzz".to_string();
    }

    number.to_string()
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
    fn test_fizzbuzz_3_returns_fizz() {
        assert_eq!(fizzbuzz(3i8), "fizz");
    }

    #[test]
    fn test_fizzbuzz_4_returns_4() {
        assert_eq!(fizzbuzz(4i8), "4");
    }

    #[test]
    fn test_fizzbuzz_5_returns_buzz() {
        assert_eq!(fizzbuzz(5i8), "buzz");
    }

    #[test]
    fn test_fizzbuzz_15_returns_buz() {
        assert_eq!(fizzbuzz(15i8), "fizzbuzz");
    }
}

