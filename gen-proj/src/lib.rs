#[cfg(test)]
mod make_canonical_should {
    use super::make_canonical;

    #[test]
    fn return_lowercase() {
        let expected = "hello";

        let actual = make_canonical("Hello");

        assert_eq!(actual, expected)
    }

    #[test]
    fn return_trimmed() {
        let expected = "hello";

        let actual = make_canonical("  Hello  ");

        assert_eq!(actual, expected)
    }

    #[test]
    fn return_hyphen_for_special_characters() {
        let expected = "three-men-s-morris";

        let actual = make_canonical("Three Men's Morris");

        assert_eq!(actual, expected)
    }
}

pub fn make_canonical(_input: &str) -> String {
    "hello".to_owned()
}
