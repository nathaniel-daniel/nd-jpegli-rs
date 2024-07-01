use crate::c_char;
use crate::j_decompress_ptr;

pub type InitSourceFn = Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>;

extern "C" {
    pub fn nd_jpegli_create_err_str(err_str: *const c_char) -> *mut c_char;

    pub fn nd_jpegli_rust_src(cinfo: j_decompress_ptr) -> *mut c_char;
}
