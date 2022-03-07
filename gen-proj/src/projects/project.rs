#[cfg(test)]
mod test_project {
    mod new_should {
        use super::super::Project;
        use crate::projects::{language::Language, project_name};

        #[test]
        fn return_project() {
            let expected = Project {
                name: project_name::ProjectName::new("Hello World").unwrap(),
                language: Language::Rust,
            };

            let actual = Project::new("Hello World", Language::Rust);

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_project_with_different_name() {
            let expected = Project {
                name: project_name::ProjectName::new("Klondike").unwrap(),
                language: Language::Rust,
            };

            let actual = Project::new("Klondike", Language::Rust);

            assert_eq!(actual, expected);
        }
    }
}

use crate::projects::{language, project_name};

#[derive(Debug, PartialEq)]
pub struct Project {
    name: project_name::ProjectName,
    language: language::Language,
}

impl Project {
    pub fn new(name: &str, _language: language::Language) -> Self {
        Project {
            name: project_name::ProjectName::new(name).unwrap(),
            language: language::Language::Rust,
        }
    }
}
