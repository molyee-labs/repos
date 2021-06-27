use colored::Colorize;

use crate::args::InitOpts;
use crate::error::{Error, Result};
use crate::fs::save;
use crate::prettify::Prettify;
use crate::repo::Repo;
use std::env;
use std::fs;
use std::os::unix::fs::symlink;

pub fn handle(opts: InitOpts) -> Result<()> {
    let mut dir = match opts.root {
        Some(dir) => dir,
        None => ".".into()
    };
    dir = dir.canonicalize()?;
    if !dir.is_dir() {
        return Err(Error::new(format!("Not a directory {}", dir.pretty())));
    }
    println!("Initializing {}", dir.pretty());
    let mut links = dir.clone();
    links.push(".repos");
    if !links.exists() {
        fs::create_dir(&links)?;
    }
    for d in fs::read_dir(&dir)? {
        let e = d?;
        let path = e.path();
        if path == links {
            continue;
        }
        match Repo::read(&path) {
            Ok(repo) => {
                let mut link = links.clone();
                link.push(repo.name());
                if link.exists() {
                    let l = fs::read_link(&link)?;
                    assert_eq!(l, path);
                    println!("{} {}", "Skip".yellow().bold(), repo.name());
                } else {
                    println!("{} {}", "Link".white(), repo.pretty());
                    symlink(repo.path(), link)?;
                }
            }
            Err(e) => {
                println!("{} {}: {}", "Skip".red().bold(), path.pretty(), e);
            }
        }
    }
    Ok(())
}

