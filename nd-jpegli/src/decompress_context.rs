use crate::Error;
use crate::ErrorString;
use nd_jpegli_sys::jpegli_decompress_struct;
use nd_jpegli_sys::nd_jpegli_create_decompress;
use nd_jpegli_sys::nd_jpegli_destroy_decompress;
use std::mem::MaybeUninit;

/// A context for decompression.
pub struct DecompressContext {
    ctx: jpegli_decompress_struct,
}

impl DecompressContext {
    /// Make a new decompress context.
    pub fn new() -> Result<Self, Error> {
        let mut ctx: MaybeUninit<jpegli_decompress_struct> = std::mem::MaybeUninit::uninit();

        let ctx = unsafe {
            let err_str = nd_jpegli_create_decompress(ctx.as_mut_ptr());
            let err_str = ErrorString::from_ptr(err_str);
            if let Some(err_str) = err_str {
                return Err(err_str.into());
            }

            let mut ctx = ctx.assume_init();
            let client_data = Box::new(ClientData {});
            let client_data_ptr = Box::into_raw(client_data);
            ctx.client_data = client_data_ptr.cast();

            ctx
        };

        Ok(Self { ctx })
    }
}

impl Drop for DecompressContext {
    fn drop(&mut self) {
        unsafe {
            let client_data_ptr = self.ctx.client_data;
            if !client_data_ptr.is_null() {
                let _client_data: Box<ClientData> = Box::from_raw(client_data_ptr.cast());
            }

            let err_str = nd_jpegli_destroy_decompress(&mut self.ctx);
            let _err_str = ErrorString::from_ptr(err_str);
        }
    }
}

/// Client data for decompressing
struct ClientData {}
