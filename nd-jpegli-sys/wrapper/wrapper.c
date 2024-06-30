#include <lib/jpegli/decode.h>
#include <lib/jpegli/encode.h>
#include <setjmp.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define SETUP_ERROR_HANDLING(cinfo)                                            \
  struct nd_jpegli_error_mgr error_mgr;                                        \
  error_mgr.err_str = NULL;                                                    \
  cinfo->err = jpegli_std_error(&error_mgr.pub);                               \
  error_mgr.pub.error_exit = error_mgr_error_exit;                             \
  error_mgr.pub.output_message = error_mgr_output_message;                     \
  error_mgr.pub.emit_message = error_mgr_emit_message;                         \
  if (setjmp(error_mgr.setjmp_buffer)) {                                       \
    return error_mgr.err_str;                                                  \
  }

#define ND_JPEGLI_ERR_MSG_MAX_SIZE JMSG_LENGTH_MAX

struct nd_jpegli_error_mgr {
  struct jpeg_error_mgr pub;
  jmp_buf setjmp_buffer;
  char *volatile err_str;
};

typedef struct nd_jpegli_error_mgr *nd_jpegli_error_mgr_ptr;

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
  if (new_err_str == NULL) {
    // Failed to allocate string buffer.
    // There isn't a lot we can do here.
    return;
  }

  (*cinfo->err->format_message)(cinfo, new_err_str);
  error_mgr->err_str = new_err_str;
}

void error_mgr_emit_message(j_common_ptr cinfo, int msg_level) {
  // TODO: Give control of this to user somehow.
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