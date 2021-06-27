use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1", author = "Alex Sarapulov <1molyee@gmail.com")]
pub struct Opts {
    #[clap(short, long, default_value = ".repos")]
    config: PathBuf
    #[clap(subcommand)]
    command: Command
}

#[derive(Clap)]
pub struct Command {
    Update(UpdateOpts),
    Clone(CloneOpts),
    Fetch(FetchOpts),
}

#[derive(Clap)]
pub struct UpdateOpts {
    
}