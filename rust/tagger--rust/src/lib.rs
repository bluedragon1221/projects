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

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, TaggerError>;

#[derive(Debug)]
pub struct TaggedFile {
    pub og_file: PathBuf,
    dir: String,
    name: String,
    tags: Vec<String>,
    extension: Option<String>,
}

fn bad_osstr(osstr: &OsStr) -> Result<String> {
    Ok(osstr
        .to_str()
        .ok_or(TaggerError::Other(
            "Couldn't convert osstr to String".into(),
        ))?
        .to_string())
}

impl TaggedFile {
    pub fn from_filename(filename: &str) -> Result<TaggedFile> {
        let path = Path::new(filename);
        if !path.exists() {
            return Err(TaggerError::Nonexistent(filename.into()));
        }

        let extension: Option<String> = if let Some(ext) = path.extension() {
            Some(
                ext.to_str()
                    .map(String::from)
                    .ok_or(TaggerError::Other("parsing".into()))?,
            )
        } else {
            None
        };

        let stem: String = bad_osstr(
            path.file_stem()
                .ok_or(TaggerError::MalformedFile(filename.into()))?,
        )?;

        let mut split_string = stem.split("--").map(String::from);

        Ok(TaggedFile {
            og_file: path.to_owned(),
            dir: path
                .parent()
                .map(|x| match (x.to_owned(), x.exists()) {
                    (a, true) => format!("{}/", a.into_os_string().into_string().unwrap()),
                    (_, false) => String::from(""),
                })
                .unwrap(),
            name: split_string
                .next()
                .ok_or(TaggerError::MalformedFile(filename.into()))?,
            tags: split_string.collect(),
            extension,
        })
    }

    pub fn generate_filename(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        res.push(self.dir.clone());
        res.push(self.name.clone());
        for i in self.tags.iter() {
            res.push(format!("--{}", i))
        }

        if let Some(ext) = &self.extension {
            res.push(format!(".{}", ext));
        };

        res.join("")
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag)
        }
    }
}
