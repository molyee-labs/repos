use crate::error::*;
use std::path::Path;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use log::debug;
use serde::{Deserialize, Serialize};

pub fn load<T: for<'de> Deserialize<'de>>(fpath: &Path) -> Result<T> {
    let format = Format::try_from(fpath)?;
    let mut r = File::open(fpath).map(BufReader::new)?;
    let data = match format {
        Format::Toml => {
            let mut s = String::new();
            r.read_to_string(&mut s)?;
            toml::from_str(&s)?
        }
    };
    Ok(data)
}

pub fn save<T: Serialize>(fpath: &Path, data: &T) -> Result<()> {
    let format = Format::try_from(fpath)?;
    let mut w = File::create(fpath).map(BufWriter::new)?;
    match format {
        Format::Toml => {
            let s = toml::to_string(data)?;
            dbg!(&s);
            w.write_all(s.as_bytes())?;
        }
    }
    Ok(())
}

pub fn is_git_repo(path: &Path) -> bool {
    let mut git = path.to_owned();
    git.push(".git");
    git.is_dir() && git.exists()
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    Toml,
}

impl TryFrom<&Path> for Format {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let ext = path.extension()
            .and_then(OsStr::to_str)
            .expect("No file extension");
        toml::from_str(ext).map_err(Into::into)
    }
}
