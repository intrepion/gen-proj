#[cfg(test)]
mod test_projects {
    mod default_should {
        use super::super::Project;

        #[test]
        fn return_project_with_defaults() {
            let _expected = Project {};
        }
    }
}

pub struct Project {}
