use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};

type OptionVec<T> = Option<Vec<T>>;

/// Directory represents directory tree, that may contain subdirectories and files.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub files: OptionVec<File>,
    pub directories: OptionVec<Directory>,
}

impl Directory {
    /// Walk thru the children elements and collect them.
    pub fn walk(&self) -> Vec<DirEntry> {
        let mut tree: Vec<DirEntry> = vec![];
        let root_name = Rc::new(self.name.clone());

        if self.files.is_none() && self.directories.is_none() {
            tree.push(
                DirEntry::new(EntryType::Directory, self.name.clone())
            );
            return tree;
        }

        if self.directories.is_some() {
            for dir in self.directories.as_ref().unwrap() {
                let mut ch = dir.walk();
                tree.append(&mut ch);
            }

        }

        if self.files.is_some() {
            for file in self.files.as_ref().unwrap() {
                tree.push(
                    DirEntry::new(EntryType::File, file.name.clone())
                );
            }
        }

        tree = tree
                .iter()
                .map(|entry|{
                    DirEntry::new(
                        entry.entry_type,
                        Rc::clone(&root_name).to_string() + "/" + &entry.full_path
                    )
                })
                .collect();
        tree
    }

    /// children_names returns names of the child dirs and files
    pub fn children_names(&self) -> Vec<String> {
        let mut children: Vec<String> = vec![];

        let mut dnames: Vec<String> = self
            .child_dirs_names();
        
        let mut fnames = self.child_files_names();
        
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
    /// file with required content
    pub content: Option<String>,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// EntryType is a type of entity contained in the directory.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EntryType {
    Directory,
    File,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct DirEntry {
    pub entry_type: EntryType,
    pub full_path: String
}

impl DirEntry {
    pub fn new(entry_type: EntryType, full_path: String) -> Self {
        DirEntry{
            entry_type,
            full_path,
        }
    }

    pub fn is_file(&self) -> bool {
        self.entry_type == EntryType::File
    }

    pub fn is_dir(&self) -> bool {
        self.entry_type == EntryType::Directory
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{Directory, File, DirEntry, EntryType};
    #[test]
    fn test_child_files() {
        let dir = Directory{
            name: "test_dir".to_string(),
            directories: None,
            files: Some(vec![
                File{name: "file.rs".to_string(), content: None}
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
                File{name: "file.rs".to_string(), content: None}
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
                                content: Some("main.tmpl".to_owned())
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
            files: Some(vec![
                File {
                    name: ".gitignore".to_string(),
                    content: None
                }
            ]),
        };
        let got = dir.walk();
        let want = vec![
            DirEntry::new(EntryType::File, String::from("td/td_td0/td0_1/test_file.rs")),
            DirEntry::new(EntryType::Directory, String::from("td/td_td1/td1_1")),
            DirEntry::new(EntryType::Directory, String::from("td/td_td2/td2_0")),
            DirEntry::new(EntryType::Directory, String::from("td/td_td2/td2_1")),
            DirEntry::new(EntryType::File, String::from("td/.gitignore")),
        ];

        assert_eq!(want, got);
    }
}
