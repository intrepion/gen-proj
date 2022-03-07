#[cfg(test)]
mod test_project_name {
    mod new_project_name_should {
        use super::super::ProjectName;

        #[test]
        fn return_trimmed_project_name() {
            let expected = ProjectName("Hello World".to_owned());

            let actual = ProjectName::new("  Hello World    ");

            assert_eq!(actual, expected);
        }
    }
}

pub struct ProjectName(String);

impl ProjectName {
    pub fn new(_raw: &str) -> ProjectName {
        ProjectName("".to_owned())
    }
}
