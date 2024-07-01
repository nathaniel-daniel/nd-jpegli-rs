mod decompress_context;
mod error_string;

pub use self::decompress_context::DecompressContext;
pub use self::decompress_context::ReadSource;
pub use self::decompress_context::Source;
pub use self::error_string::ErrorString;

/// An error that may occur while using this library.
#[derive(Debug)]
pub enum Error {
    /// A jpegli error string
    Jpegli(ErrorString),

    /// An Api error occured.
    ///
    /// You, as the programmer, did something wrong.
    Api(&'static str),

    /// Encountered something that is currently unsupported.
    ///
    /// Feel free to open a bug.
    Unsupported(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Jpegli(_error) => write!(f, "jpegli error"),
            Self::Api(error) => write!(f, "api error (\"{error}\")"),
            Self::Unsupported(error) => write!(f, "unsupported error (\"{error}\")"),
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

/// The color space of a jpeg
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ColorSpace {
    Luma,
    Rgb,
    YCbCr,
    Cmyk,
    Ycck,

    Unknown,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decompress() {
        let file =
            std::fs::File::open("Plush_bunny_with_headphones.jpg").expect("failed to open file");
        let read_source = ReadSource::new(file);

        let mut ctx = DecompressContext::new(read_source).expect("failed to create context");
        ctx.read_header().expect("failed to read headers");
        let image_dimensions = ctx.image_dimensions().expect("missing dimensions");
        let color_space = ctx.jpeg_color_space().expect("missing color space");
        assert!(image_dimensions == (800, 533));
        assert!(color_space == ColorSpace::YCbCr);

        drop(ctx);
    }
}
