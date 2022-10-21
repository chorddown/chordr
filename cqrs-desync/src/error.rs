use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
#[derive(Debug)]
pub enum Error {
    Path(&'static str, Option<IoError>),
    Read(&'static str, IoError),
    Io(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Path(message, _) => f.write_str(message),
            Error::Read(message, _) => f.write_str(message),
            Error::Io(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Path(_, _inner) => None, //  todo!(), // inner.as_ref(),
            Error::Read(_, inner) => Some(inner),
            Error::Io(inner) => Some(inner),
        }
    }
}
impl From<IoError> for Error {
    fn from(inner: IoError) -> Self {
        Self::Io(inner)
    }
}
