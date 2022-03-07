#[cfg(test)]
mod test_project {
    mod new_should {
        use super::super::Project;
        use crate::projects::project_name;

        #[test]
        fn return_project() {
            let expected = Project {
                _name: project_name::ProjectName::new("Hello World").unwrap(),
            };

            let actual = Project::new("Hello World");

            assert_eq!(actual, expected);
        }
    }
}

use crate::projects::project_name;

#[derive(Debug, PartialEq)]
pub struct Project {
    _name: project_name::ProjectName,
}

impl Project {
    pub fn new(raw: &str) -> Self {
        Project {
            _name: project_name::ProjectName::new(raw).unwrap(),
        }
    }
}
