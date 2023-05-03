use crate::data::Entry::*;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Formatter, Write};

type Path = String;

#[derive(Eq, PartialEq, Debug)]
pub enum Entry {
    Directory(String),
    File(String, usize),
}

#[derive(Eq, PartialEq, Default)]
pub struct FileSystem {
    paths: BTreeMap<Path, Entry>,
}

impl FileSystem {
    pub fn new(lines: Vec<String>) -> Self {
        let mut fs = FileSystem::default();

        let mut current_dir = "".to_string();
        let mut currently_ls = false;

        for line in lines {
            if line.starts_with("$ cd") {
                currently_ls = false;

                let cd = line.split(' ').last().unwrap();
                match cd {
                    "/" => current_dir = "".to_string(),
                    ".." => current_dir.truncate(current_dir.rfind('/').unwrap()),
                    _ => {
                        if !current_dir.ends_with('/') {
                            current_dir.push('/');
                        }
                        current_dir.push_str(cd);
                    }
                }
            } else if line.starts_with("$ ls") {
                currently_ls = true;
            } else if currently_ls {
                if line.starts_with("dir") {
                    let dir_name = line.split(' ').last().unwrap();
                    let dir_path = current_dir.clone() + "/" + dir_name;
                    fs.add_entry(dir_path, Directory(dir_name.to_string()));
                } else {
                    let mut file_split = line.split(' ');
                    let file_size = file_split.next().unwrap().parse().unwrap();
                    let file_name = file_split.next().unwrap();
                    let file_path = current_dir.clone() + "/" + file_name;
                    fs.add_entry(file_path, File(file_name.to_string(), file_size));
                }
            } else {
                panic!("Unknown line: {}", line);
            }
        }

        fs
    }

    pub fn add_entry(&mut self, path: Path, entry: Entry) {
        self.paths.insert(path, entry);
    }

    pub fn dir_sizes(&self) -> HashMap<Path, usize> {
        let mut dirs = HashMap::new();

        for (path, entry) in self.paths.iter().rev() {
            match entry {
                Directory(_name) => {
                    dirs.entry(path.to_owned()).or_insert(0);
                }
                File(_name, size) => {
                    let mut dir = path.clone();

                    // Traverse up the directory tree, adding the file's size
                    while let Some(last_slash_idx) = dir.rfind('/') {
                        dir.truncate(last_slash_idx);
                        *dirs.entry(dir.clone()).or_insert(0) += size;
                    }
                }
            }
        }

        dirs
    }
}

impl Debug for FileSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{\n")?;

        for (path, entry) in self.paths.iter() {
            let line = match entry {
                Directory(_name) => format!("  D {}", path),
                File(_name, size) => format!("  F {} - {}", path, size),
            };
            f.write_str(&line)?;
            f.write_char('\n')?;
        }

        f.write_char('}')
    }
}
