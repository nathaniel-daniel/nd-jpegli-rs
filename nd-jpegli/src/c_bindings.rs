use nd_jpegli_sys::c_char;
use nd_jpegli_sys::j_decompress_ptr;

extern "C" {
    pub(crate) fn nd_jpegli_create_err_str(err_str: *const c_char) -> *mut c_char;
    pub(crate) fn nd_jpegli_rust_src(cinfo: j_decompress_ptr) -> *mut c_char;
}
