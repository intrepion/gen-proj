#[cfg(test)]
mod test_project {
    mod default_should {
        use super::super::Project;

        #[test]
        fn return_project_with_defaults() {
            let _expected = Project {};

            let _actual = Project::new("Hello World");
        }
    }
}

pub struct Project {}
