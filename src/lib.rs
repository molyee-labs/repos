use args::Opts;
mod error;
mod fs;
mod git;
mod repo;
mod args;
mod commands;
pub(crate) mod prettify;

use crate::error::Result;
use crate::commands::*;
use crate::args::Command;
use clap::Clap;

pub fn handle() -> Result<()> {
    let opts = Opts::parse();
    match opts.command {
        Command::Init(t) => init::handle(t),
        Command::Add(s) => add::handle(s),
        Command::Remove(b) => remove::handle(b),
    }
}