use serde::{Deserialize, Serialize};
use std::{fs, io, rc::Rc};

type OptionVec<T> = Option<Vec<T>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Directory {
    name: String,
    files: OptionVec<String>,
    directories: OptionVec<Directory>,
}

impl Directory {
    pub fn walk(&self) -> Vec<String> {
        if self.directories.is_some() {
            let mut tree = vec![];
            let root_name = Rc::new(self.name.clone());

            for dir in self.directories.as_ref().unwrap() {
                let mut ch = dir.walk();
                tree.append(&mut ch);
            }

            tree = tree
                .iter()
                .map(|ch_dirs| {
                    Rc::clone(&root_name).to_string() + "/" + ch_dirs.as_str()
                })
                .collect();
            tree
        } else {
            vec![self.name.clone()]
        }
    }

    /// self_and_children returns name of the directory and this child dirs names
    pub fn self_and_children(&self) -> Vec<String> {
        let name = self.name.clone();
        let name_ref = Rc::new(name);

        let cn: Vec<String> = self
            .child_dirs_names()
            .clone()
            .iter()
            .map(|x| {
                Rc::clone(&name_ref).to_string() + "/" + x.as_str()
            })
            .collect();
        cn
    }

    /// child_dirs_names returns names of child directories
    pub fn child_dirs_names(&self) -> Vec<String> {
        let mut ch_ns: Vec<String> = vec![];
        if self.directories.is_some() {
            for d in self.directories.as_ref().unwrap() {
                ch_ns.push(d.name.clone());
            }
        }
        ch_ns
    }
}

/// Template represents YAML template structure
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    directories: Vec<Directory>,
    files: OptionVec<String>,
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
    use crate::{Directory, Template};

    #[test]
    fn walk_dir() {
        let dir = Directory {
            name: "td".to_string(),
            directories: Some(vec![
                Directory {
                    name: "td_td0".to_string(),
                    directories: Some(vec![Directory {
                        name: "td0_1".to_string(),
                        directories: None,
                        files: None,
                    }]),
                    files: None,
                },
                Directory {
                    name: "td_td1".to_string(),
                    directories: None,
                    files: None,
                },
                Directory {
                    name: "td_td2".to_string(),
                    directories: Some(vec![
                        Directory{
                            name: String::from("td2_0"),
                            directories: None,
                            files: None
                        },
                        Directory{
                            name: String::from("td2_1"),
                            directories: None,
                            files: None
                        }
                    ]),
                    files: None,
                },
            ]),
            files: None,
        };
        let v = dir.walk();

        assert_eq!(v, vec![
            "td/td_td0/td0_1",
            "td/td_td1",
            "td/td_td2/td2_0",
            "td/td_td2/td2_1"
        ]);
    }

    #[test]
    fn create_template() {
        let yaml = "
            directories:
                - directory:
                  name: test
        ";
        assert!(Template::new(yaml).is_ok());
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
