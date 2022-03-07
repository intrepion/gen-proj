#[cfg(test)]
mod test_language {
    mod from_string_should {
        use super::super::Language;
        use std::str::FromStr;

        #[test]
        fn return_language_given_rust_language() {
            let expected = Ok(Language::Rust);

            let actual = Language::from_str("rust");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_language_given_capitalized_language() {
            let expected = Ok(Language::Rust);

            let actual = Language::from_str("Rust");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_language_given_extra_spaces_language() {
            let expected = Ok(Language::Rust);

            let actual = Language::from_str("   rust     ");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_err_given_unknown_language() {
            let expected = Err("Unknown language: unknown".to_owned());

            let actual = Language::from_str("unknown");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_language_given_closure_language() {
            let expected = Ok(Language::Closure);

            let actual = Language::from_str("closure");

            assert_eq!(actual, expected);
        }
    }
}

use std::str;

#[derive(Debug, PartialEq)]
pub enum Language {
    Closure,
    Rust,
}

impl str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}
