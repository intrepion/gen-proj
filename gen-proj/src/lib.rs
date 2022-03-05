#[cfg(test)]
mod make_canonical_should {
    use super::make_canonical;

    #[test]
    fn return_lowercase() {
        let _expected = "hello";
        let _actual = make_canonical("Hello");
    }
}

pub fn make_canonical(input: &str) -> String {
    input.to_lowercase()
}
