#[allow(
    non_camel_case_types,
    clippy::upper_case_acronyms,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
pub(crate) mod bindings;

pub use self::bindings::boolean;
pub use self::bindings::j_compress_ptr;
pub use self::bindings::j_decompress_ptr;
pub use self::bindings::jpeg_compress_struct as jpegli_compress_struct;
pub use self::bindings::jpeg_decompress_struct as jpegli_decompress_struct;
pub use self::bindings::jpeg_error_mgr as jpegli_error_mgr;
pub use self::bindings::jpeg_error_mgr__bindgen_ty_1 as jpegli_error_mgr_msg_parm;
pub use self::bindings::jpeg_memory_mgr as jpegli_memory_manager;
pub use self::bindings::jpeg_progress_mgr as jpegli_progress_mgr;
pub use self::bindings::jpeg_source_mgr as jpegli_source_mgr;
pub use self::bindings::JDIMENSION;
pub use self::bindings::JPEG_HEADER_OK as JPEGLI_HEADER_OK;
pub use self::bindings::JPEG_HEADER_TABLES_ONLY as JPEGLI_HEADER_TABLES_ONLY;
pub use self::bindings::JPEG_SUSPENDED as JPEGLI_SUSPENDED;
pub use self::bindings::JSAMPARRAY;
pub use self::bindings::J_COLOR_SPACE;
pub use self::bindings::J_DCT_METHOD;
pub use core::ffi::c_char;
pub use core::ffi::c_int;
pub use core::ffi::c_uint;
pub use core::ffi::c_ulong;

pub const JPEG_EOI: u8 = 0xD9;
pub const FALSE: boolean = 0;
pub const TRUE: boolean = 1;

extern "C" {
    /// Initialize a new compress context.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_create_compress(ctx: j_compress_ptr) -> *mut c_char;

    /// Setup a compress context to use the a buffer as a file destination.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_mem_dest(
        ctx: j_compress_ptr,
        buffer: *mut *mut u8,
        buffer_len: *mut c_ulong,
    ) -> *mut c_char;

    /// Set default parameters for a compress operation.
    ///
    /// Needs at least in_color_space to be specified beforehand.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_set_defaults(ctx: j_compress_ptr) -> *mut c_char;

    /// Start compressing.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_start_compress(ctx: j_compress_ptr) -> *mut c_char;

    /// Write scanlines to a compress context.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_write_scanlines(
        ctx: j_compress_ptr,
        scanlines: JSAMPARRAY,
        num_lines: JDIMENSION,
        ret: *mut JDIMENSION,
    ) -> *mut c_char;

    /// Finish compressing.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_finish_compress(ctx: j_compress_ptr) -> *mut c_char;

    /// Destroy a compress context.
    pub fn nd_jpegli_destroy_compress(ctx: j_compress_ptr);

    /// Initialize a new decompress context.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_create_decompress(ctx: j_decompress_ptr) -> *mut c_char;

    /// Setup a decompress context to use the a buffer as a file source.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_mem_src(
        ctx: j_decompress_ptr,
        buffer: *const u8,
        buffer_len: c_ulong,
    ) -> *mut c_char;

    /// Read a jpeg's headers, and set compression parameters.
    ///
    /// # Parameters
    /// `j_decompress_ptr`: A pointer to the decompress context.
    /// `ret`: A pointer to the return value of jpegli_read_header. This can be `JPEG_SUSPENDED`, `JPEG_HEADER_OK`, or `JPEG_HEADER_TABLES_ONLY`.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_read_header(ctx: j_decompress_ptr, ret: *mut c_uint) -> *mut c_char;

    /// Start decompressing.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_start_decompress(ctx: j_decompress_ptr, ret: *mut boolean) -> *mut c_char;

    /// Read scanlines from a decompress context.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_read_scanlines(
        ctx: j_decompress_ptr,
        scanlines: JSAMPARRAY,
        max_lines: JDIMENSION,
        ret: *mut JDIMENSION,
    ) -> *mut c_char;

    /// Finish decompression.
    ///
    /// # Returns
    /// If successful, returns NULL.
    /// Otherwise, returns a pointer to a NUL-terminated c-string.
    /// This c-string is threadsafe, and must be freed with `nd_jpegli_free_err_str`.
    pub fn nd_jpegli_finish_decompress(ctx: j_decompress_ptr, ret: *mut boolean) -> *mut c_char;

    /// Destroy a decompress context.
    pub fn nd_jpegli_destroy_decompress(ctx: j_decompress_ptr);

    /// Free an error string allocated by this library.
    pub fn nd_jpegli_free_err_str(err_str: *mut c_char);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn decompress() {
        let buffer = std::fs::read("Plush_bunny_with_headphones.jpg").expect("failed to read file");

        unsafe {
            let mut ctx: MaybeUninit<jpegli_decompress_struct> = std::mem::MaybeUninit::zeroed();
            let err = nd_jpegli_create_decompress(ctx.as_mut_ptr());
            assert!(err.is_null());

            let mut ctx = ctx.assume_init();

            let buffer = &buffer[..];
            let err = nd_jpegli_mem_src(
                &mut ctx,
                buffer.as_ptr(),
                buffer.len().try_into().expect("buffer too large"),
            );
            assert!(err.is_null());

            let mut ret = JPEGLI_HEADER_OK;
            let err = nd_jpegli_read_header(&mut ctx, &mut ret);
            assert!(err.is_null());
            assert!(ret == JPEGLI_HEADER_OK);

            assert!(ctx.image_width == 800);
            assert!(ctx.image_height == 533);
            assert!(ctx.num_components == 3);
            assert!(ctx.jpeg_color_space == J_COLOR_SPACE::JCS_YCbCr);

            let mut ret = FALSE;
            let err = nd_jpegli_start_decompress(&mut ctx, &mut ret);
            assert!(err.is_null());
            assert!(ret == TRUE);
            assert!(ctx.output_width == 800);
            assert!(ctx.output_height == 533);
            assert!(ctx.output_components == 3);
            assert!(ctx.out_color_space == J_COLOR_SPACE::JCS_RGB);

            let row_stride = usize::try_from(ctx.output_width).unwrap()
                * usize::try_from(ctx.output_components).unwrap();
            let output_height = usize::try_from(ctx.output_height).unwrap();
            let mut scanline_buffer = vec![0; row_stride * output_height];

            while ctx.output_scanline < ctx.output_height {
                let output_scanline = usize::try_from(ctx.output_scanline).unwrap();
                let scanline_buffer = &mut scanline_buffer[(output_scanline * row_stride)..];
                assert!(scanline_buffer.len() >= row_stride);

                let mut scanlines_read = 0;
                let err = nd_jpegli_read_scanlines(
                    &mut ctx,
                    &mut scanline_buffer.as_mut_ptr(),
                    1,
                    &mut scanlines_read,
                );
                assert!(err.is_null());
                assert!(scanlines_read == 1);
            }

            let mut ret = FALSE;
            let err = nd_jpegli_finish_decompress(&mut ctx, &mut ret);
            assert!(err.is_null());

            let img =
                image::RgbImage::from_vec(ctx.output_width, ctx.output_height, scanline_buffer)
                    .unwrap();

            img.save("test-decompress.jpeg").unwrap();

            nd_jpegli_destroy_decompress(&mut ctx);
        }
    }

    #[test]
    fn compress() {
        let image = image::open("Plush_bunny_with_headphones.jpg").expect("failed to read image");
        let image = image.as_rgb8().expect("failed to convert to rgb8");
        let image_buffer = image.clone().into_vec();

        let mut out_buffer = Vec::with_capacity(1_000_000);

        unsafe {
            let mut ctx: MaybeUninit<jpegli_compress_struct> = std::mem::MaybeUninit::zeroed();
            let err = nd_jpegli_create_compress(ctx.as_mut_ptr());
            assert!(err.is_null());

            let mut ctx = ctx.assume_init();

            let mut out_buffer_ptr = out_buffer.as_mut_ptr();
            let mut out_buffer_len = out_buffer.capacity().try_into().expect("buffer too large");
            let err = nd_jpegli_mem_dest(&mut ctx, &mut out_buffer_ptr, &mut out_buffer_len);
            assert!(err.is_null());

            ctx.image_width = image.width();
            ctx.image_height = image.height();
            ctx.input_components = 3;
            ctx.in_color_space = J_COLOR_SPACE::JCS_RGB;

            // Set default parameters
            let err = nd_jpegli_set_defaults(&mut ctx);
            assert!(err.is_null());

            let err = nd_jpegli_start_compress(&mut ctx);
            assert!(err.is_null());

            let row_stride = ctx.image_width * 3;
            let row_stride_usize = usize::try_from(row_stride).unwrap();
            while ctx.next_scanline < ctx.image_height {
                let mut num_scanlines_written = 0;
                let scanline_start = usize::try_from(ctx.next_scanline).unwrap() * row_stride_usize;
                let scanline_end = scanline_start + row_stride_usize;

                let scanline = &image_buffer[scanline_start..scanline_end];
                let mut scanline_ptr = scanline.as_ptr() as *mut u8;
                let err = nd_jpegli_write_scanlines(
                    &mut ctx,
                    &mut scanline_ptr,
                    1,
                    &mut num_scanlines_written,
                );
                assert!(err.is_null());
                assert!(num_scanlines_written == 1);
            }

            let err = nd_jpegli_finish_compress(&mut ctx);
            assert!(err.is_null());

            assert!(out_buffer_len != 0);
            out_buffer.set_len(out_buffer_len.try_into().unwrap());

            nd_jpegli_destroy_compress(&mut ctx);
        }

        std::fs::write("test-compress.jpeg", out_buffer).expect("failed to write");
    }
}
