#include "wrapper.h"
#include <assert.h>
#include <string.h>

void error_mgr_error_exit(j_common_ptr cinfo) {
  nd_jpegli_error_mgr_ptr error_mgr = (nd_jpegli_error_mgr_ptr)cinfo->err;
  (*cinfo->err->output_message)(cinfo);
  longjmp(error_mgr->setjmp_buffer, 1);
}

void error_mgr_output_message(j_common_ptr cinfo) {
  nd_jpegli_error_mgr_ptr error_mgr = (nd_jpegli_error_mgr_ptr)cinfo->err;

  // Free old message.
  // Only write the last message.
  if (error_mgr->err_str != NULL) {
    free(error_mgr->err_str);
    error_mgr->err_str = NULL;
  }

  // Allocate buffer
  char *new_err_str = malloc(ND_JPEGLI_ERR_MSG_MAX_SIZE);
  assert(new_err_str != NULL);

  (*cinfo->err->format_message)(cinfo, new_err_str);
  error_mgr->err_str = new_err_str;
}

void error_mgr_emit_message(j_common_ptr cinfo, int msg_level) {
  // TODO: Give control of this to user somehow.
  (void)cinfo;
  (void)msg_level;
}

// Init a compress context.
char *nd_jpegli_create_compress(j_compress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_create_compress(cinfo);

  return NULL;
}

/// Setup a compress context to use the a buffer as a file destination.
char *nd_jpegli_mem_dest(j_compress_ptr cinfo, unsigned char **outbuffer,
                         unsigned long *outsize) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_mem_dest(cinfo, outbuffer, outsize);

  return NULL;
}

/// Set default compress options.
char *nd_jpegli_set_defaults(j_compress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_set_defaults(cinfo);

  return NULL;
}

/// Set the compression quality.
char *nd_jpegli_set_quality(j_compress_ptr cinfo, int quality,
                            boolean force_baseline) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_set_quality(cinfo, quality, force_baseline);

  return NULL;
}

/// Set xyb mode.
char *nd_jpegli_set_xyb_mode(j_compress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_set_xyb_mode(cinfo);

  return NULL;
}

/// Start compressing.
char *nd_jpegli_start_compress(j_compress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_start_compress(cinfo, TRUE);

  return NULL;
}

/// Write scanlines.
char *nd_jpegli_write_scanlines(j_compress_ptr cinfo, JSAMPARRAY scanlines,
                                JDIMENSION num_lines, JDIMENSION *ret) {
  SETUP_ERROR_HANDLING(cinfo);

  *ret = jpegli_write_scanlines(cinfo, scanlines, num_lines);

  return NULL;
}

/// Finish compression.
char *nd_jpegli_finish_compress(j_compress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_finish_compress(cinfo);

  return NULL;
}

/// Destroy a compress context.
void nd_jpegli_destroy_compress(j_compress_ptr cinfo) {
  jpegli_destroy_compress(cinfo);
}

/// Init a decompress context.
char *nd_jpegli_create_decompress(j_decompress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_create_decompress(cinfo);

  return NULL;
}

/// Setup a decompress context to use the a buffer as a file source.
char *nd_jpegli_mem_src(j_decompress_ptr cinfo, const unsigned char *inbuffer,
                        unsigned long insize) {
  SETUP_ERROR_HANDLING(cinfo);

  jpegli_mem_src(cinfo, inbuffer, insize);

  return NULL;
}

/// Read a jpeg's headers, and set compression parameters.
char *nd_jpegli_read_header(j_decompress_ptr cinfo, unsigned int *ret) {
  SETUP_ERROR_HANDLING(cinfo);

  *ret = jpegli_read_header(cinfo, TRUE);

  return NULL;
}

/// Start decompressing.
char *nd_jpegli_start_decompress(j_decompress_ptr cinfo, boolean *ret) {
  SETUP_ERROR_HANDLING(cinfo);

  *ret = jpegli_start_decompress(cinfo);

  return NULL;
}

/// Read scanlines from a decompress context.
char *nd_jpegli_read_scanlines(j_decompress_ptr cinfo, JSAMPARRAY scanlines,
                               JDIMENSION max_lines, JDIMENSION *ret) {
  SETUP_ERROR_HANDLING(cinfo);

  *ret = jpegli_read_scanlines(cinfo, scanlines, max_lines);

  return NULL;
}

/// Finish decompressing.
char *nd_jpegli_finish_decompress(j_decompress_ptr cinfo, boolean *ret) {
  SETUP_ERROR_HANDLING(cinfo);

  *ret = jpegli_finish_decompress(cinfo);

  return NULL;
}

/// Destroy a decompress context.
void nd_jpegli_destroy_decompress(j_decompress_ptr cinfo) {
  jpegli_destroy_decompress(cinfo);
}

/// Free an error string.
void nd_jpegli_free_err_str(char *err_str) {
  if (err_str == NULL) {
    return;
  }

  free(err_str);
}