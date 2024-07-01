use crate::ColorSpace;
use crate::Error;
use crate::ErrorString;
use core::ffi::c_void;
use nd_jpegli_sys::__private_nd_jpegli_rs::nd_jpegli_create_err_str;
use nd_jpegli_sys::__private_nd_jpegli_rs::nd_jpegli_rust_src;
use nd_jpegli_sys::c_char;
use nd_jpegli_sys::c_uint;
use nd_jpegli_sys::j_decompress_ptr;
use nd_jpegli_sys::jpegli_decompress_struct;
use nd_jpegli_sys::nd_jpegli_create_decompress;
use nd_jpegli_sys::nd_jpegli_destroy_decompress;
use nd_jpegli_sys::nd_jpegli_read_header;
use nd_jpegli_sys::nd_jpegli_read_scanlines;
use nd_jpegli_sys::nd_jpegli_start_decompress;
use nd_jpegli_sys::FALSE;
use nd_jpegli_sys::JPEGLI_HEADER_OK;
use nd_jpegli_sys::JPEG_EOI;
use nd_jpegli_sys::TRUE;
use smallvec::SmallVec;
use std::io::Read;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

#[no_mangle]
pub extern "C" fn nd_jpegli_rust_src_init_source_rs(ctx: j_decompress_ptr) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        // `client_data` should have been initialized in the constructor and
        // We currently have unique access to this struct.
        let client_data: &mut ClientData = unsafe { &mut *(*ctx).client_data.cast::<ClientData>() };

        client_data.source.as_mut().init_source()
    });

    match result {
        Ok(Ok(())) => std::ptr::null_mut(),
        Ok(Err(mut error_string)) => {
            error_string.push('\0');
            unsafe { nd_jpegli_create_err_str(error_string.as_ptr().cast()) }
        }
        Err(_panic) => unsafe { nd_jpegli_create_err_str("panic\0".as_ptr().cast()) },
    }
}

#[no_mangle]
pub extern "C" fn nd_jpegli_rust_src_fill_input_buffer_rs(ctx: j_decompress_ptr) -> *mut c_char {
    let result = std::panic::catch_unwind(|| {
        // `client_data` should have been initialized in the constructor and
        // We currently have unique access to this struct.
        let client_data: &mut ClientData = unsafe { &mut *(*ctx).client_data.cast::<ClientData>() };

        client_data
            .source
            .as_mut()
            .fill_input_buffer(&mut client_data.source_buffer)
            .map(|n| (client_data.source_buffer.as_ptr(), n))
    });

    match result {
        Ok(Ok((ptr, n))) => {
            unsafe {
                (*(*ctx).src).next_input_byte = ptr;
                (*(*ctx).src).bytes_in_buffer = n;
            }

            std::ptr::null_mut()
        }
        Ok(Err(mut error_string)) => {
            error_string.push('\0');
            unsafe { nd_jpegli_create_err_str(error_string.as_ptr().cast()) }
        }
        Err(_panic) => unsafe { nd_jpegli_create_err_str("panic\0".as_ptr().cast()) },
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum State {
    Initial,
    Header,
    StartDecompress,
    Error,
}

/// A context for decompression.
#[repr(C)]
pub struct DecompressContext<S> {
    ctx: jpegli_decompress_struct,
    state: State,
    source: PhantomData<Box<S>>,
}

impl<S> DecompressContext<S>
where
    S: Source + 'static,
{
    /// Make a new decompress context.
    pub fn new(source: S) -> Result<Self, Error> {
        let mut ctx: MaybeUninit<jpegli_decompress_struct> = std::mem::MaybeUninit::zeroed();

        // Setup ctx
        let ctx = unsafe {
            let err_str = nd_jpegli_create_decompress(ctx.as_mut_ptr());
            let err_str = ErrorString::from_ptr(err_str);
            if let Some(err_str) = err_str {
                return Err(err_str.into());
            }

            let mut ctx = ctx.assume_init();
            let client_data = Box::new(ClientData {
                source: Box::new(source),
                source_buffer: vec![0; 4096],
            });
            let client_data_ptr: *mut ClientData = Box::into_raw(client_data);
            ctx.client_data = client_data_ptr.cast::<c_void>();

            ctx
        };

        // Create this early.
        // This will ensure we drop the struct and client data appropriately on error.
        // This is safe even if we fail to init the source.
        // Source initialization only sets up the vtable with the trampoline fns.
        let mut ctx = Self {
            ctx,
            state: State::Initial,
            source: PhantomData,
        };

        // Setup Source
        unsafe {
            let err_str = nd_jpegli_rust_src(&mut ctx.ctx);
            let err_str = ErrorString::from_ptr(err_str);
            if let Some(err_str) = err_str {
                return Err(err_str.into());
            }
        }

        Ok(ctx)
    }

    /// Read the header.
    pub fn read_header(&mut self) -> Result<(), Error> {
        if self.state != State::Initial {
            return Err(Error::Api("cannot call from this state"));
        }

        let mut ret = JPEGLI_HEADER_OK;
        unsafe {
            let err_str = nd_jpegli_read_header(&mut self.ctx, &mut ret);
            let err_str = ErrorString::from_ptr(err_str);
            if let Some(err_str) = err_str {
                self.state = State::Error;
                return Err(err_str.into());
            }
        }

        if ret != JPEGLI_HEADER_OK {
            self.state = State::Error;
            return Err(Error::Unsupported(
                "`read_header` return code not supported",
            ));
        }

        self.state = State::Header;
        Ok(())
    }

    /// Start decompression.
    pub fn start_decompress(&mut self) -> Result<(), Error> {
        if self.state != State::Header {
            return Err(Error::Api("cannot call from this state"));
        }

        let mut ret = FALSE;
        unsafe {
            let err_str = nd_jpegli_start_decompress(&mut self.ctx, &mut ret);
            let err_str = ErrorString::from_ptr(err_str);
            if let Some(err_str) = err_str {
                self.state = State::Error;
                return Err(err_str.into());
            }
        }

        if ret != TRUE {
            self.state = State::Error;
            return Err(Error::Unsupported("source suspension is not supported"));
        }

        self.state = State::StartDecompress;

        Ok(())
    }

    /// Read scanlines into a list of scanlines.
    pub fn read_scanlines(&mut self, scanlines: &mut [&mut [u8]]) -> Result<u32, Error> {
        if self.state != State::StartDecompress {
            return Err(Error::Api("cannot call from this state"));
        }

        let num_scanlines = match c_uint::try_from(scanlines.len()) {
            Ok(num_scanlines) => num_scanlines,
            Err(_error) => {
                return Err(Error::Api("scanline buffer buffer too large"));
            }
        };

        let row_stride = usize::try_from(self.ctx.output_width).unwrap()
            * usize::try_from(self.ctx.output_components).unwrap();

        let mut scanlines_arg: SmallVec<[*mut u8; 32]> = SmallVec::with_capacity(scanlines.len());
        for scanline in scanlines.iter_mut() {
            if scanline.len() < row_stride {
                return Err(Error::Api("a scanline buffer was too small"));
            }
            scanlines_arg.push(scanline.as_mut_ptr());
        }

        let mut scanlines_read = 0;
        unsafe {
            let err_str = nd_jpegli_read_scanlines(
                &mut self.ctx,
                scanlines_arg.as_mut_ptr(),
                num_scanlines,
                &mut scanlines_read,
            );
            let err_str = ErrorString::from_ptr(err_str);
            if let Some(err_str) = err_str {
                self.state = State::Error;
                return Err(err_str.into());
            }
        }

        if scanlines_read == 0 {
            self.state = State::Error;
            return Err(Error::Api("source suspension is not supported"));
        }

        #[allow(clippy::useless_conversion)]
        Ok(u32::try_from(scanlines_read).unwrap())
    }

    /// Read the input width.
    pub fn input_width(&self) -> Option<u32> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some(u32::try_from(self.ctx.image_width).unwrap())
    }

    /// Read the input height.
    pub fn input_height(&self) -> Option<u32> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some(u32::try_from(self.ctx.image_height).unwrap())
    }

    // Read the input width and height.
    pub fn input_dimensions(&self) -> Option<(u32, u32)> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some((
            u32::try_from(self.ctx.image_width).unwrap(),
            u32::try_from(self.ctx.image_height).unwrap(),
        ))
    }

    /// Read input the color space.
    pub fn input_color_space(&self) -> Option<ColorSpace> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        Some(ColorSpace::from(self.ctx.jpeg_color_space))
    }

    /// Read the # of components in the input colorspace.
    pub fn input_components(&self) -> Option<u8> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        Some(u8::try_from(self.ctx.num_components).unwrap())
    }

    /// Read the output width.
    pub fn output_width(&self) -> Option<u32> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some(u32::try_from(self.ctx.output_width).unwrap())
    }

    /// Read the output height.
    pub fn output_height(&self) -> Option<u32> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some(u32::try_from(self.ctx.output_height).unwrap())
    }

    /// Read the output dimensions.
    pub fn output_dimensions(&self) -> Option<(u32, u32)> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some((
            u32::try_from(self.ctx.output_width).unwrap(),
            u32::try_from(self.ctx.output_height).unwrap(),
        ))
    }

    /// Read the # of components in the output colorspace.
    pub fn output_components(&self) -> Option<u8> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        Some(u8::try_from(self.ctx.output_components).unwrap())
    }

    /// Read the output color space.
    pub fn output_color_space(&self) -> Option<ColorSpace> {
        if !matches!(self.state, State::Header | State::StartDecompress) {
            return None;
        }

        Some(ColorSpace::from(self.ctx.out_color_space))
    }

    /// Read the number of scanlines this library has returned.
    pub fn output_scanline(&self) -> Option<u32> {
        if !matches!(self.state, State::StartDecompress) {
            return None;
        }

        #[allow(clippy::useless_conversion)]
        Some(u32::try_from(self.ctx.output_scanline).unwrap())
    }
}

impl<S> Drop for DecompressContext<S> {
    fn drop(&mut self) {
        unsafe {
            let client_data_ptr = self.ctx.client_data;
            if !client_data_ptr.is_null() {
                let client_data: Box<ClientData> = Box::from_raw(client_data_ptr.cast());
                drop(client_data);
            }

            nd_jpegli_destroy_decompress(&mut self.ctx);
        }
    }
}

/// Client data for decompressing
struct ClientData {
    source: Box<dyn Source>,
    source_buffer: Vec<u8>,
}

/// A data source
pub trait Source {
    fn init_source(&mut self) -> Result<(), String>;
    fn fill_input_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, String>;
}

/// A source around Rust's Read trait.
pub struct ReadSource<R> {
    reader: R,
    start_of_file: bool,
}

impl<R> ReadSource<R> {
    /// Make a new read source.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            start_of_file: false,
        }
    }
}

impl<R> Source for ReadSource<R>
where
    R: Read,
{
    fn init_source(&mut self) -> Result<(), String> {
        self.start_of_file = true;
        Ok(())
    }

    fn fill_input_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, String> {
        let mut n = self
            .reader
            .read(buffer)
            .map_err(|error| error.to_string())?;

        if n == 0 {
            if self.start_of_file {
                return Err(String::from("input empty")); /* Treat empty input file as fatal error */
            }

            /* Insert a fake EOI marker */
            buffer[0] = 0xFF;
            buffer[1] = JPEG_EOI;
            n = 2;
        }

        self.start_of_file = false;

        Ok(n)
    }
}
