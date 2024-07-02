#include "wrapper.h"
#include <string.h>

#define INPUT_BUF_SIZE 4096

extern char *nd_jpegli_rust_src_init_source_rs(j_decompress_ptr cinfo);
extern char *nd_jpegli_rust_src_fill_input_buffer_rs(j_decompress_ptr cinfo);

void nd_jpegli_rust_src_init_source_c(j_decompress_ptr cinfo) {
  nd_jpegli_error_mgr_ptr error_mgr = (nd_jpegli_error_mgr_ptr)cinfo->err;

  char *ret = nd_jpegli_rust_src_init_source_rs(cinfo);

  if (ret != NULL) {
    error_mgr->err_str = ret;
    longjmp(error_mgr->setjmp_buffer, 1);
  }
}

boolean nd_jpegli_rust_src_fill_input_buffer_c(j_decompress_ptr cinfo) {
  nd_jpegli_error_mgr_ptr error_mgr = (nd_jpegli_error_mgr_ptr)cinfo->err;

  char *ret = nd_jpegli_rust_src_fill_input_buffer_rs(cinfo);

  if (ret != NULL) {
    error_mgr->err_str = ret;
    longjmp(error_mgr->setjmp_buffer, 1);
  }

  return TRUE;
}

void nd_jpegli_rust_src_skip_input_data_c(j_decompress_ptr cinfo,
                                          long num_bytes) {
  struct jpeg_source_mgr *src = cinfo->src;

  /* Just a dumb implementation for now.  Could use fseek() except
   * it doesn't work on pipes.  Not clear that being smart is worth
   * any trouble anyway --- large skips are infrequent.
   */
  if (num_bytes > 0) {
    while (num_bytes > (long)src->bytes_in_buffer) {
      num_bytes -= (long)src->bytes_in_buffer;
      (void)(*src->fill_input_buffer)(cinfo);
      /* note we assume that fill_input_buffer will never return FALSE,
       * so suspension need not be handled.
       */
    }
    src->next_input_byte += (size_t)num_bytes;
    src->bytes_in_buffer -= (size_t)num_bytes;
  }
}

void nd_jpegli_rust_src_term_source(j_decompress_ptr cinfo) { (void)cinfo; }

char *nd_jpegli_create_err_str(const char *err_str) {
  // Allocate buffer
  char *new_err_str = malloc(ND_JPEGLI_ERR_MSG_MAX_SIZE);
  assert(new_err_str != NULL);

  strcpy(new_err_str, err_str);

  return new_err_str;
}

char *nd_jpegli_rust_src(j_decompress_ptr cinfo) {
  SETUP_ERROR_HANDLING(cinfo);

  if (cinfo->src != NULL) {
    return nd_jpegli_create_err_str(
        "nd_jpegli_rust_src: a different source manager was already set");
  }

  cinfo->src = (struct jpeg_source_mgr *)(*cinfo->mem->alloc_small)(
      (j_common_ptr)cinfo, JPOOL_PERMANENT, sizeof(struct jpeg_source_mgr));

  cinfo->src->init_source = nd_jpegli_rust_src_init_source_c;
  cinfo->src->fill_input_buffer = nd_jpegli_rust_src_fill_input_buffer_c;
  cinfo->src->skip_input_data = nd_jpegli_rust_src_skip_input_data_c;
  cinfo->src->resync_to_restart =
      jpegli_resync_to_restart; /* use default method */
  cinfo->src->term_source = nd_jpegli_rust_src_term_source;
  cinfo->src->bytes_in_buffer = 0;
  cinfo->src->next_input_byte = NULL;

  return NULL;
}