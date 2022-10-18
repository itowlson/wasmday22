#ifndef __BINDINGS_CONSOLE_H
#define __BINDINGS_CONSOLE_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } console_string_t;
  
  void console_string_set(console_string_t *ret, const char *s);
  void console_string_dup(console_string_t *ret, const char *s);
  void console_string_free(console_string_t *ret);
  void console_handle_console_input(console_string_t *line, console_string_t *ret0);
  #ifdef __cplusplus
}
#endif
#endif
