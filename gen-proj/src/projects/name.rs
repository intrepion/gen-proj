#[cfg(test)]
mod test_name {
    mod new_name_should {
        use super::super::Name;

        #[test]
        fn return_name() {
            let expected = Ok(Name("Klondike".to_owned()));

            let actual = Name::new("Klondike");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_trimmed_name() {
            let expected = Ok(Name("Hello World".to_owned()));

            let actual = Name::new("  Hello World    ");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_none_if_empty_string() {
            let expected = Err("Project name cannot be empty".to_owned());

            let actual = Name::new("");

            assert_eq!(actual, expected);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Name(String);

impl Name {
    pub fn new(raw: &str) -> Result<Self, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err("Project name cannot be empty".to_owned());
        }

        Ok(Name(trimmed.to_owned()))
    }
}
