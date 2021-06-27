pub struct GitRepo {

}

impl Git {
    pub fn from_path(repo: T) -> Self {
        Self { repo }
    }

    pub fn fetch(&self) -> Result<()> {

    }
}

