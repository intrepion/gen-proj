#[cfg(test)]
mod test_utilities {
    mod make_canonical_should {
        use super::super::make_canonical;

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
}

use regex::Regex;

pub fn make_canonical(input: &str) -> String {
    let lowercased = input.to_lowercase();
    let trimmed = lowercased.trim();
    let re = Regex::new(r"[^A-Za-z0-9_]").unwrap();
    re.replace_all(trimmed, "-").into_owned()
}
