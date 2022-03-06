#[cfg(test)]
mod languages {
    mod from_string_should {
        use super::super::Language;

        #[test]
        fn return_rust_enum_given_rust_string() {
            let _expected = Language::Rust;
        }
    }
}

pub enum Language {
    Rust,
}
