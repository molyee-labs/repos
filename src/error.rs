use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("{0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("{0}")]
    Custom(String)
}

impl Error {
    pub fn new(msg: String) -> Self {
        Self::Custom(msg)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::new(s.to_owned())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
