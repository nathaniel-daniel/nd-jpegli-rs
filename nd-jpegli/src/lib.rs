mod decompress_context;
mod error_string;

pub use self::error_string::ErrorString;
pub use crate::decompress_context::DecompressContext;

/// An error that may occur while using this library.
#[derive(Debug)]
pub enum Error {
    /// A jpegli error string
    Jpegli(ErrorString),

    /// An Api error occured.
    ///
    /// You, as the programmer, did something wrong.
    Api(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Jpegli(_error) => write!(f, "jpegli error"),
            Self::Api(error) => write!(f, "api error (\"{error}\")"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Jpegli(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ErrorString> for Error {
    fn from(error: ErrorString) -> Self {
        Self::Jpegli(error)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decompress() {
        let ctx = DecompressContext::new(&[]);
        drop(ctx);
    }
}
