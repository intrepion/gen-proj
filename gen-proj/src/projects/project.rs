#[cfg(test)]
mod test_project {
    mod new_should {
        use super::super::Project;
        use crate::projects::{language::Language, name};

        #[test]
        fn return_hello_world_rust_project() {
            let expected = Project {
                name: name::Name::new("Hello World").unwrap(),
                language: Language::Rust,
            };

            let actual = Project::new("Hello World", Language::Rust);

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_klondike_rust_project() {
            let expected = Project {
                name: name::Name::new("Klondike").unwrap(),
                language: Language::Rust,
            };

            let actual = Project::new("Klondike", Language::Rust);

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_hello_world_closure_project() {
            let expected = Project {
                name: name::Name::new("Hello World").unwrap(),
                language: Language::Closure,
            };

            let actual = Project::new("Hello World", Language::Closure);

            assert_eq!(actual, expected);
        }
    }
}

use crate::projects::{language, name};

#[derive(Debug, PartialEq)]
pub struct Project {
    name: name::Name,
    language: language::Language,
}

impl Project {
    pub fn new(name: &str, language: language::Language) -> Self {
        Project {
            name: name::Name::new(name).unwrap(),
            language,
        }
    }
}
