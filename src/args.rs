use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1", author = "Alex Sarapulov <1molyee@gmail.com")]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Clap)]
pub enum Command {
    Init(InitOpts),
    Add(AddOpts),
    Remove(RemoveOpts)
}

#[derive(Clap)]
pub struct InitOpts {
    #[clap(parse(from_os_str))]
    pub root: Option<PathBuf>
}

#[derive(Clap)]
pub struct AddOpts {
    #[clap(parse(from_str))]
    pub name: String,
    #[clap(long)]
    pub remote: Option<String>,
}

#[derive(Clap)]
pub struct RemoveOpts {
    #[clap(parse(from_str))]
    pub name: String,
    #[clap(long)]
    pub remote: Option<String>,
}
