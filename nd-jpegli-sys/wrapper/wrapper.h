#ifndef __ND_JPEGLI_WRAPPER__
#define __ND_JPEGLI_WRAPPER__

#include <assert.h>
#include <lib/jpegli/decode.h>
#include <lib/jpegli/encode.h>
#include <setjmp.h>
#include <stdlib.h>

#define ND_JPEGLI_ERR_MSG_MAX_SIZE JMSG_LENGTH_MAX

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

struct nd_jpegli_error_mgr {
  struct jpeg_error_mgr pub;
  jmp_buf setjmp_buffer;
  char *volatile err_str;
};

typedef struct nd_jpegli_error_mgr *nd_jpegli_error_mgr_ptr;

void error_mgr_error_exit(j_common_ptr cinfo);
void error_mgr_output_message(j_common_ptr cinfo);
void error_mgr_emit_message(j_common_ptr cinfo, int msg_level);

#endif