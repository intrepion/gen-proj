#[cfg(test)]
mod languages {
    mod from_string_should {
        use super::super::Language;
        use std::str::FromStr;

        #[test]
        fn return_rust_enum_given_rust_string() {
            let _expected = Language::Rust;
            let _actual = Language::from_str("rust");
        }
    }
}

use std::str;

pub enum Language {
    Rust,
}

impl str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(format!("Unknown language: {}", s))
    }
}
