#![feature(os_str_display)]

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum TaggerError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Wrong number of arguments: {0}")]
    ArgumentError(usize),

    #[error("Malformed filename: {0}")]
    MalformedFile(String),

    #[error("File doesn't exist: {0}")]
    Nonexistent(String),
}

pub type Result<T> = std::result::Result<T, TaggerError>;

#[derive(Debug, Clone)]
pub struct TaggedFile {
    parent: PathBuf,
    name: String,
    tags: Vec<String>,
    extension: Option<String>,
}

pub fn dsp(str: &OsStr) -> String {
    str.display().to_string()
}

impl TaggedFile {
    pub fn from_filename(filename: &str) -> Result<TaggedFile> {
        let path = Path::new(filename);
        if !path.exists() {
            return Err(TaggerError::Nonexistent(filename.into()));
        }

        let stem: String = path
            .file_stem()
            .map(dsp) // if Some, convert to String
            .unwrap_or("".into());

        let extension: Option<String> = path.extension().map(dsp);

        let mut split_string = stem.split("--").map(String::from);
        let name = split_string.next().unwrap_or("".into());
        let tags = split_string.collect();

        Ok(TaggedFile {
            parent: path.parent().unwrap().to_path_buf(),
            name,
            tags,
            extension,
        })
    }

    pub fn generate_filename(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        res.push(self.parent.join(self.name.clone()).display().to_string());
        for i in self.tags.iter() {
            res.push(format!("--{}", i))
        }

        if let Some(ext) = &self.extension {
            res.push(format!(".{}", ext));
        };

        res.join("")
    }

    fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag)
        }
    }

    fn rm_tag(&mut self, tag: String) {
        self.tags.retain(|x| *x != tag);
    }

    pub fn new_with_modlist(&self, modlist: &[String]) -> Self {
        let mut new_file = self.clone();
        for i in modlist {
            let mut chars = i.chars();
            match chars.next() {
                Some('+') => new_file.add_tag(chars.collect()),
                Some('-') => new_file.rm_tag(chars.collect()),
                Some(_) => new_file.add_tag(i.to_string()),
                None => (),
            }
        }

        new_file
    }
}
