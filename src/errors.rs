use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    Yarsg(String),
    Tera(tera::Error),
    Io(::std::io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
}

#[derive(Debug)]
pub struct Error {
    /// Kind of error
    pub kind: ErrorKind,
    pub source: Option<Box<dyn StdError + Send + Sync>>,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.source {
            Some(ref err) => Some(&**err),
            None => match self.kind {
                ErrorKind::Tera(ref err) => err.source(),
                _ => None,
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Yarsg(ref message) => write!(f, "{}", message),
            ErrorKind::Tera(ref e) => write!(f, "{}", e),
            ErrorKind::Io(ref e) => write!(f, "{}", e),
            ErrorKind::TomlDe(ref e) => write!(f, "{}", e),
            ErrorKind::TomlSer(ref e) => write!(f, "{}", e),
        }
    }
}

impl Error {
    pub fn new(msg: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Yarsg(msg.to_string()),
            source: None,
        }
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::new(e)
    }
}
impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::new(e)
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self { kind: ErrorKind::TomlDe(e), source: None }
    }
}
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self { kind: ErrorKind::TomlSer(e), source: None }
    }
}
impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Self {
        Self { kind: ErrorKind::Tera(e), source: None }
    }
}
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Self { kind: ErrorKind::Io(e), source: None }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;