mod c_bindings;
mod decompress_context;
mod error_string;

pub use self::decompress_context::DecompressContext;
pub use self::decompress_context::ReadSource;
pub use self::decompress_context::Source;
pub use self::error_string::ErrorString;
use nd_jpegli_sys::J_COLOR_SPACE;

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

impl From<J_COLOR_SPACE> for ColorSpace {
    fn from(color_space: J_COLOR_SPACE) -> Self {
        match color_space {
            J_COLOR_SPACE::JCS_UNKNOWN => ColorSpace::Unknown,
            J_COLOR_SPACE::JCS_GRAYSCALE => ColorSpace::Luma,
            J_COLOR_SPACE::JCS_RGB => ColorSpace::Rgb,
            J_COLOR_SPACE::JCS_YCbCr => ColorSpace::YCbCr,
            J_COLOR_SPACE::JCS_CMYK => ColorSpace::Cmyk,
            J_COLOR_SPACE::JCS_YCCK => ColorSpace::Ycck,
            _ => ColorSpace::Unknown,
        }
    }
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
        assert!(ctx.input_width() == Some(800));
        assert!(ctx.input_height() == Some(533));
        assert!(ctx.input_dimensions() == Some((800, 533)));
        assert!(ctx.input_color_space() == Some(ColorSpace::YCbCr));
        assert!(ctx.input_components() == Some(3));

        ctx.start_decompress()
            .expect("failed to start decompression");
        let output_width = ctx.output_width().expect("missing output width");
        let output_height = ctx.output_height().expect("missing output height");
        let output_components = ctx.output_components().expect("missing output components");
        assert!(output_width == 800);
        assert!(output_height == 533);
        assert!(ctx.output_color_space() == Some(ColorSpace::Rgb));
        assert!(ctx.output_components() == Some(3));

        let row_stride = usize::try_from(output_width).unwrap() * usize::from(output_components);
        let output_height_usize = usize::try_from(output_height).unwrap();
        let mut scanline_buffer = vec![0; row_stride * output_height_usize];

        loop {
            let output_scanline = ctx.output_scanline().expect("failed to get scanline");
            if output_scanline >= output_height {
                break;
            }

            let output_scanline = usize::try_from(output_scanline).unwrap();
            let scanline_buffer = &mut scanline_buffer[(output_scanline * row_stride)..];
            assert!(scanline_buffer.len() >= row_stride);

            ctx.read_scanlines(&mut [scanline_buffer])
                .expect("failed to read scanlines");
        }

        drop(ctx);

        let img = image::RgbImage::from_vec(output_width, output_height, scanline_buffer).unwrap();

        img.save("test-decompress.jpeg").unwrap();
    }
}
