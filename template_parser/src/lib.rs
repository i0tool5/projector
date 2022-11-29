use serde::{Deserialize, Serialize};
use std::{fs, io};

pub mod entities;

type OptionVec<T> = Option<Vec<T>>;

/// Template represents YAML template structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub template_files_dirs: Option<String>,
    pub directories: Vec<entities::Directory>,
    pub files: OptionVec<String>,
}

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let result = fs::read_to_string(path)?;
    Ok(result)
}

impl Template {
    pub fn new(yaml: &str) -> Result<Self, serde_yaml::Error> {
        let t = serde_yaml::from_str(yaml)?;
        Ok(t)
    }

    pub fn parse_template(&self) {
        let _dirs = self.directories.iter();
        todo!();
    }

    pub fn generate(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Template};
    #[test]
    fn create_template() {
        let yaml = "
            template_files_dirs: test_files
            directories:
                - directory:
                  name: test
        ";
        let templ = Template::new(yaml);
        assert!(templ.is_ok());
        let templ_data = templ.unwrap();
        assert_ne!(templ_data.directories.len(), 0);
        assert!(templ_data.template_files_dirs.is_some());
        let files_dirs = templ_data.template_files_dirs.unwrap();
        assert_eq!(files_dirs, String::from("test_files"));
    }

    #[test]
    fn parse_template() {
        let yaml = "
            directories:
                - directory:
                  name: test0
                - directory:
                  name: test1
                  directories:
                    - directory:
                      name: test1_test0
                    - directory:
                      name: test1_test1
                    - directory:
                      name: test1_test2
                      directories:
                        - directory:
                          name: test1_test2_test0
        ";
        let template = Template::new(yaml).unwrap();
        template.parse_template();
    }
}
