#[cfg(test)]
mod make_canonical_should {
    use super::make_canonical;

    #[test]
    fn return_lowercase() {
        let expected = "hello";

        let actual = make_canonical("Hello");

        assert_eq!(actual, expected)
    }
}

pub fn make_canonical(_input: &str) -> String {
    "".to_owned()
}
