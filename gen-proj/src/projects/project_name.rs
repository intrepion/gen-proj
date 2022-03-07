#[cfg(test)]
mod test_project_name {
    mod new_project_name_should {
        use super::super::ProjectName;

        #[test]
        fn return_trimmed_project_name() {
            let _expected = ProjectName("Hello World".to_owned());

            let _actual = ProjectName::new("  Hello World    ".to_owned());
        }
    }
}

pub struct ProjectName(String);
