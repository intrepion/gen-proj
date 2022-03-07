#[cfg(test)]
mod test_project_name {
    mod new_project_name_should {
        #[test]
        fn return_trimmed_project_name() {
            let expected = ProjectName("Hello World".to_owned());
        }
    }
}
