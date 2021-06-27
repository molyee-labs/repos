use repos;
use log::error;
use colored::*;

fn main() {
    env_logger::init();
    if let Err(e) = repos::handle() {
        error!("{}", e);
        eprintln!("{}: {}", "Error".bright_red().bold(), e.to_string());
    }
}
