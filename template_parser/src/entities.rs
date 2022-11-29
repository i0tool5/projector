use serde::{Deserialize, Serialize};
use std::{rc::Rc, fmt::Display};

type OptionVec<T> = Option<Vec<T>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub files: OptionVec<File>,
    pub directories: OptionVec<Directory>,
}

impl Directory {
    pub fn walk(&self) -> Vec<String> {
        let mut tree = vec![];
        let root_name = Rc::new(self.name.clone());

        if self.directories.is_some() {
            for dir in self.directories.as_ref().unwrap() {
                let mut ch = dir.walk();
                tree.append(&mut ch);
            }

        } else if self.files.is_some() {
            for file in self.files.as_ref().unwrap() {
                tree.push(file.name.clone());
            }
        } else {
            tree.push(self.name.clone());
            return tree;
        }
        tree = tree
                .iter()
                .map(|ch_name|{
                    Rc::clone(&root_name).to_string() + "/" + ch_name.as_str()
                })
                .collect();
        tree
    }

    /// children_names returns names of the child dirs and files
    pub fn children_names(&self) -> Vec<String> {
        let mut children: Vec<String> = vec![];

        let mut dnames: Vec<String> = self
            .child_dirs_names();
            // .clone()
            // .iter()
            // .map(|x| {
            //     Rc::clone(&name_ref).to_string() + "/" + x.as_str()
            // })
            // .collect();
        
        let mut fnames = self.child_files_names();
        
        dbg!(fnames.clone());
        
        children.append(&mut dnames);
        children.append(&mut fnames);

        children
    }

    /// child_dirs_names returns names of child directories
    pub fn child_dirs_names(&self) -> Vec<String> {
        let mut children: Vec<String> = vec![];
        if self.directories.is_some() {
            for entity in self.directories.as_ref().unwrap() {
                children.push(entity.name.clone());
            }
        }
        children
    }

    pub fn child_files_names(&self) -> Vec<String> {
        let mut children: Vec<String> = vec![];
        if self.files.is_some() {
            for entity in self.files.as_ref().unwrap() {
                children.push(entity.name.clone());
            }
        }
        children
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub content_file: Option<String>
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{Directory, File};
    #[test]
    fn test_child_files() {
        let dir = Directory{
            name: "test_dir".to_string(),
            directories: None,
            files: Some(vec![
                File{name: "file.rs".to_string(), content_file: None}
            ])
        };
        let fname = dir.child_files_names();
        assert_eq!(vec!["file.rs".to_string()], fname);
    }
    #[test]
    fn test_get_child_names() {
        let dir = Directory{
            name: "test_dir".to_string(),
            directories: Some(vec![
                Directory{name: "test".to_string(), directories: None, files: None}
            ]),
            files: Some(vec![
                File{name: "file.rs".to_string(), content_file: None}
            ])
        };
        let ch = dir.children_names();
        assert_eq!(vec!["test".to_string(), "file.rs".to_string()], ch);
    }
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
                        files: Some(vec![
                            File {
                                name: "test_file.rs".to_string(),
                                content_file: None
                            }
                        ]),
                    }]),
                    files: None,
                },
                Directory {
                    name: "td_td1".to_string(),
                    directories: Some(vec![Directory {
                        name: "td1_1".to_string(),
                        directories: None,
                        files: None
                    }]),
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
            "td/td_td0/td0_1/test_file.rs",
            "td/td_td1/td1_1",
            "td/td_td2/td2_0",
            "td/td_td2/td2_1"
        ]);
    }
}
