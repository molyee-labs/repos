use crate::error::{Error, Result};
use crate::fs;
use crate::prettify::Prettify;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use colored::{ColoredString, Colorize};

pub struct Repo {
    path: PathBuf
}

impl Repo {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        if fs::is_git_repo(path.as_ref()) {
            let path = path.as_ref().to_owned();
            Ok(Self { path })
        } else {
            Err(Error::from("Unknown repository"))
        }
    }

    pub fn name(&self) -> &str {
        self.path.file_name().and_then(OsStr::to_str).unwrap()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn is_git(&self) -> bool {
        fs::is_git_repo(&self.path)
    }

    pub fn as_git(&self) -> Result<GitRepo<'_>> {
        if is_git(self) {
            Ok(Git::new(self.path()))
        } else {
            Err(Error::from("Not a git repository"))
        }
    }
}

impl Prettify for Repo {
    type Out = ColoredString;
    fn pretty(&self) -> Self::Out {
        self.name().bright_green().bold()
    }
}

