#[cfg(test)]
mod test_project_name {
    mod new_project_name_should {
        use super::super::ProjectName;

        #[test]
        fn return_project_name() {
            let expected = Some(ProjectName("Klondike".to_owned()));

            let actual = ProjectName::new("Klondike");

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_trimmed_project_name() {
            let expected = Some(ProjectName("Hello World".to_owned()));

            let actual = ProjectName::new("  Hello World    ");

            assert_eq!(actual, expected);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(raw: &str) -> ProjectName {
        ProjectName(raw.trim().to_owned())
    }
}
