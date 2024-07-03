/* automatically generated by rust-bindgen 0.69.4 */

pub const JPEG_SUSPENDED: u32 = 0;
pub const JPEG_HEADER_OK: u32 = 1;
pub const JPEG_HEADER_TABLES_ONLY: u32 = 2;
pub type JSAMPLE = ::std::os::raw::c_uchar;
pub type JCOEF = ::std::os::raw::c_short;
pub type JOCTET = ::std::os::raw::c_uchar;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type JDIMENSION = ::std::os::raw::c_uint;
pub type boolean = ::std::os::raw::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JBLOCK = [JCOEF; 64usize];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64usize],
    pub sent_table: boolean,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17usize],
    pub huffval: [UINT8; 256usize],
    pub sent_table: boolean,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_component_info {
    pub component_id: ::std::os::raw::c_int,
    pub component_index: ::std::os::raw::c_int,
    pub h_samp_factor: ::std::os::raw::c_int,
    pub v_samp_factor: ::std::os::raw::c_int,
    pub quant_tbl_no: ::std::os::raw::c_int,
    pub dc_tbl_no: ::std::os::raw::c_int,
    pub ac_tbl_no: ::std::os::raw::c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_h_scaled_size: ::std::os::raw::c_int,
    pub DCT_v_scaled_size: ::std::os::raw::c_int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: ::std::os::raw::c_int,
    pub MCU_height: ::std::os::raw::c_int,
    pub MCU_blocks: ::std::os::raw::c_int,
    pub MCU_sample_width: ::std::os::raw::c_int,
    pub last_col_width: ::std::os::raw::c_int,
    pub last_row_height: ::std::os::raw::c_int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_scan_info {
    pub comps_in_scan: ::std::os::raw::c_int,
    pub component_index: [::std::os::raw::c_int; 4usize],
    pub Ss: ::std::os::raw::c_int,
    pub Se: ::std::os::raw::c_int,
    pub Ah: ::std::os::raw::c_int,
    pub Al: ::std::os::raw::c_int,
}
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: ::std::os::raw::c_uint,
    pub data_length: ::std::os::raw::c_uint,
    pub data: *mut JOCTET,
}
impl J_COLOR_SPACE {
    pub const JCS_UNKNOWN: J_COLOR_SPACE = J_COLOR_SPACE(0);
}
impl J_COLOR_SPACE {
    pub const JCS_GRAYSCALE: J_COLOR_SPACE = J_COLOR_SPACE(1);
}
impl J_COLOR_SPACE {
    pub const JCS_RGB: J_COLOR_SPACE = J_COLOR_SPACE(2);
}
impl J_COLOR_SPACE {
    pub const JCS_YCbCr: J_COLOR_SPACE = J_COLOR_SPACE(3);
}
impl J_COLOR_SPACE {
    pub const JCS_CMYK: J_COLOR_SPACE = J_COLOR_SPACE(4);
}
impl J_COLOR_SPACE {
    pub const JCS_YCCK: J_COLOR_SPACE = J_COLOR_SPACE(5);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_RGB: J_COLOR_SPACE = J_COLOR_SPACE(6);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_RGBX: J_COLOR_SPACE = J_COLOR_SPACE(7);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_BGR: J_COLOR_SPACE = J_COLOR_SPACE(8);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_BGRX: J_COLOR_SPACE = J_COLOR_SPACE(9);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_XBGR: J_COLOR_SPACE = J_COLOR_SPACE(10);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_XRGB: J_COLOR_SPACE = J_COLOR_SPACE(11);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_RGBA: J_COLOR_SPACE = J_COLOR_SPACE(12);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_BGRA: J_COLOR_SPACE = J_COLOR_SPACE(13);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_ABGR: J_COLOR_SPACE = J_COLOR_SPACE(14);
}
impl J_COLOR_SPACE {
    pub const JCS_EXT_ARGB: J_COLOR_SPACE = J_COLOR_SPACE(15);
}
impl J_COLOR_SPACE {
    pub const JCS_RGB565: J_COLOR_SPACE = J_COLOR_SPACE(16);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct J_COLOR_SPACE(pub ::std::os::raw::c_uint);
impl J_DCT_METHOD {
    pub const JDCT_ISLOW: J_DCT_METHOD = J_DCT_METHOD(0);
}
impl J_DCT_METHOD {
    pub const JDCT_IFAST: J_DCT_METHOD = J_DCT_METHOD(1);
}
impl J_DCT_METHOD {
    pub const JDCT_FLOAT: J_DCT_METHOD = J_DCT_METHOD(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct J_DCT_METHOD(pub ::std::os::raw::c_uint);
pub const J_DITHER_MODE_JDITHER_NONE: J_DITHER_MODE = 0;
pub const J_DITHER_MODE_JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const J_DITHER_MODE_JDITHER_FS: J_DITHER_MODE = 2;
pub type J_DITHER_MODE = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::std::os::raw::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::std::os::raw::c_int,
}
pub type j_common_ptr = *mut jpeg_common_struct;
pub type j_compress_ptr = *mut jpeg_compress_struct;
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::std::os::raw::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::std::os::raw::c_int,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: ::std::os::raw::c_int,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: f64,
    pub scale_num: ::std::os::raw::c_uint,
    pub scale_denom: ::std::os::raw::c_uint,
    pub jpeg_width: JDIMENSION,
    pub jpeg_height: JDIMENSION,
    pub data_precision: ::std::os::raw::c_int,
    pub num_components: ::std::os::raw::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4usize],
    pub q_scale_factor: [::std::os::raw::c_int; 4usize],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    pub arith_dc_L: [UINT8; 16usize],
    pub arith_dc_U: [UINT8; 16usize],
    pub arith_ac_K: [UINT8; 16usize],
    pub num_scans: ::std::os::raw::c_int,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub do_fancy_downsampling: boolean,
    pub smoothing_factor: ::std::os::raw::c_int,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: ::std::os::raw::c_uint,
    pub restart_in_rows: ::std::os::raw::c_int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    pub progressive_mode: boolean,
    pub max_h_samp_factor: ::std::os::raw::c_int,
    pub max_v_samp_factor: ::std::os::raw::c_int,
    pub min_DCT_h_scaled_size: ::std::os::raw::c_int,
    pub min_DCT_v_scaled_size: ::std::os::raw::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub comps_in_scan: ::std::os::raw::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4usize],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: ::std::os::raw::c_int,
    pub MCU_membership: [::std::os::raw::c_int; 10usize],
    pub Ss: ::std::os::raw::c_int,
    pub Se: ::std::os::raw::c_int,
    pub Ah: ::std::os::raw::c_int,
    pub Al: ::std::os::raw::c_int,
    pub block_size: ::std::os::raw::c_int,
    pub natural_order: *const ::std::os::raw::c_int,
    pub lim_Se: ::std::os::raw::c_int,
    pub master: *mut jpeg_comp_master,
    pub main: *mut jpeg_c_main_controller,
    pub prep: *mut jpeg_c_prep_controller,
    pub coef: *mut jpeg_c_coef_controller,
    pub marker: *mut jpeg_marker_writer,
    pub cconvert: *mut jpeg_color_converter,
    pub downsample: *mut jpeg_downsampler,
    pub fdct: *mut jpeg_forward_dct,
    pub entropy: *mut jpeg_entropy_encoder,
    pub script_space: *mut jpeg_scan_info,
    pub script_space_size: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::std::os::raw::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::std::os::raw::c_int,
    pub src: *mut jpeg_source_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: ::std::os::raw::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: ::std::os::raw::c_uint,
    pub scale_denom: ::std::os::raw::c_uint,
    pub output_gamma: f64,
    pub buffered_image: boolean,
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    pub quantize_colors: boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: boolean,
    pub desired_number_of_colors: ::std::os::raw::c_int,
    pub enable_1pass_quant: boolean,
    pub enable_external_quant: boolean,
    pub enable_2pass_quant: boolean,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: ::std::os::raw::c_int,
    pub output_components: ::std::os::raw::c_int,
    pub rec_outbuf_height: ::std::os::raw::c_int,
    pub actual_number_of_colors: ::std::os::raw::c_int,
    pub colormap: JSAMPARRAY,
    pub output_scanline: JDIMENSION,
    pub input_scan_number: ::std::os::raw::c_int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: ::std::os::raw::c_int,
    pub output_iMCU_row: JDIMENSION,
    pub coef_bits: *mut [::std::os::raw::c_int; 64usize],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4usize],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    pub data_precision: ::std::os::raw::c_int,
    pub comp_info: *mut jpeg_component_info,
    pub is_baseline: boolean,
    pub progressive_mode: boolean,
    pub arith_code: boolean,
    pub arith_dc_L: [UINT8; 16usize],
    pub arith_dc_U: [UINT8; 16usize],
    pub arith_ac_K: [UINT8; 16usize],
    pub restart_interval: ::std::os::raw::c_uint,
    pub saw_JFIF_marker: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub saw_Adobe_marker: boolean,
    pub Adobe_transform: UINT8,
    pub CCIR601_sampling: boolean,
    pub marker_list: jpeg_saved_marker_ptr,
    pub max_h_samp_factor: ::std::os::raw::c_int,
    pub max_v_samp_factor: ::std::os::raw::c_int,
    pub min_DCT_h_scaled_size: ::std::os::raw::c_int,
    pub min_DCT_v_scaled_size: ::std::os::raw::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub sample_range_limit: *mut JSAMPLE,
    pub comps_in_scan: ::std::os::raw::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4usize],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: ::std::os::raw::c_int,
    pub MCU_membership: [::std::os::raw::c_int; 10usize],
    pub Ss: ::std::os::raw::c_int,
    pub Se: ::std::os::raw::c_int,
    pub Ah: ::std::os::raw::c_int,
    pub Al: ::std::os::raw::c_int,
    pub block_size: ::std::os::raw::c_int,
    pub natural_order: *const ::std::os::raw::c_int,
    pub lim_Se: ::std::os::raw::c_int,
    pub unread_marker: ::std::os::raw::c_int,
    pub master: *mut jpeg_decomp_master,
    pub main: *mut jpeg_d_main_controller,
    pub coef: *mut jpeg_d_coef_controller,
    pub post: *mut jpeg_d_post_controller,
    pub inputctl: *mut jpeg_input_controller,
    pub marker: *mut jpeg_marker_reader,
    pub entropy: *mut jpeg_entropy_decoder,
    pub idct: *mut jpeg_inverse_dct,
    pub upsample: *mut jpeg_upsampler,
    pub cconvert: *mut jpeg_color_deconverter,
    pub cquantize: *mut jpeg_color_quantizer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_error_mgr {
    pub error_exit: ::std::option::Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub emit_message: ::std::option::Option<
        unsafe extern "C" fn(cinfo: j_common_ptr, msg_level: ::std::os::raw::c_int),
    >,
    pub output_message: ::std::option::Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub format_message: ::std::option::Option<
        unsafe extern "C" fn(cinfo: j_common_ptr, buffer: *mut ::std::os::raw::c_char),
    >,
    pub reset_error_mgr: ::std::option::Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub msg_code: ::std::os::raw::c_int,
    pub msg_parm: jpeg_error_mgr__bindgen_ty_1,
    pub trace_level: ::std::os::raw::c_int,
    pub num_warnings: ::std::os::raw::c_long,
    pub jpeg_message_table: *const *const ::std::os::raw::c_char,
    pub last_jpeg_message: ::std::os::raw::c_int,
    pub addon_message_table: *const *const ::std::os::raw::c_char,
    pub first_addon_message: ::std::os::raw::c_int,
    pub last_addon_message: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union jpeg_error_mgr__bindgen_ty_1 {
    pub i: [::std::os::raw::c_int; 8usize],
    pub s: [::std::os::raw::c_char; 80usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: ::std::option::Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub pass_counter: ::std::os::raw::c_long,
    pub pass_limit: ::std::os::raw::c_long,
    pub completed_passes: ::std::os::raw::c_int,
    pub total_passes: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: usize,
    pub init_destination: ::std::option::Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
    pub empty_output_buffer:
        ::std::option::Option<unsafe extern "C" fn(cinfo: j_compress_ptr) -> boolean>,
    pub term_destination: ::std::option::Option<unsafe extern "C" fn(cinfo: j_compress_ptr)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: usize,
    pub init_source: ::std::option::Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
    pub fill_input_buffer:
        ::std::option::Option<unsafe extern "C" fn(cinfo: j_decompress_ptr) -> boolean>,
    pub skip_input_data: ::std::option::Option<
        unsafe extern "C" fn(cinfo: j_decompress_ptr, num_bytes: ::std::os::raw::c_long),
    >,
    pub resync_to_restart: ::std::option::Option<
        unsafe extern "C" fn(cinfo: j_decompress_ptr, desired: ::std::os::raw::c_int) -> boolean,
    >,
    pub term_source: ::std::option::Option<unsafe extern "C" fn(cinfo: j_decompress_ptr)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_memory_mgr {
    pub alloc_small: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: ::std::os::raw::c_int,
            sizeofobject: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub alloc_large: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: ::std::os::raw::c_int,
            sizeofobject: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub alloc_sarray: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: ::std::os::raw::c_int,
            samplesperrow: JDIMENSION,
            numrows: JDIMENSION,
        ) -> JSAMPARRAY,
    >,
    pub alloc_barray: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: ::std::os::raw::c_int,
            blocksperrow: JDIMENSION,
            numrows: JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: ::std::os::raw::c_int,
            pre_zero: boolean,
            samplesperrow: JDIMENSION,
            numrows: JDIMENSION,
            maxaccess: JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            pool_id: ::std::os::raw::c_int,
            pre_zero: boolean,
            blocksperrow: JDIMENSION,
            numrows: JDIMENSION,
            maxaccess: JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: ::std::option::Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub access_virt_sarray: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            ptr: jvirt_sarray_ptr,
            start_row: JDIMENSION,
            num_rows: JDIMENSION,
            writable: boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: ::std::option::Option<
        unsafe extern "C" fn(
            cinfo: j_common_ptr,
            ptr: jvirt_barray_ptr,
            start_row: JDIMENSION,
            num_rows: JDIMENSION,
            writable: boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: ::std::option::Option<
        unsafe extern "C" fn(cinfo: j_common_ptr, pool_id: ::std::os::raw::c_int),
    >,
    pub self_destruct: ::std::option::Option<unsafe extern "C" fn(cinfo: j_common_ptr)>,
    pub max_memory_to_use: ::std::os::raw::c_long,
    pub max_alloc_chunk: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_comp_master {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_c_main_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_c_prep_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_c_coef_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_marker_writer {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_color_converter {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_downsampler {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_forward_dct {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_entropy_encoder {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_decomp_master {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_d_main_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_d_coef_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_d_post_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_input_controller {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_marker_reader {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_entropy_decoder {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_inverse_dct {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_upsampler {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_color_deconverter {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_color_quantizer {
    pub _address: u8,
}
