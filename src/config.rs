pub struct Config {
    repos: Vec<Repo>
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        ...
    }

    pub fn new() -> Self {
        let repos = Vec::new();
        Config { repos }
    }


}