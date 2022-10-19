#ifndef __BINDINGS_RANDOM_THING_H
#define __BINDINGS_RANDOM_THING_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } random_thing_string_t;
  
  void random_thing_string_set(random_thing_string_t *ret, const char *s);
  void random_thing_string_dup(random_thing_string_t *ret, const char *s);
  void random_thing_string_free(random_thing_string_t *ret);
  typedef uint8_t random_thing_animal_type_t;
  #define RANDOM_THING_ANIMAL_TYPE_CAT 0
  #define RANDOM_THING_ANIMAL_TYPE_DOG 1
  typedef struct {
    uint8_t tag;
    union {
      random_thing_animal_type_t animal_fact;
    } val;
  } random_thing_request_t;
  #define RANDOM_THING_REQUEST_JOKE 0
  #define RANDOM_THING_REQUEST_ANIMAL_FACT 1
  typedef struct {
    uint8_t tag;
    union {
      random_thing_string_t network;
      uint16_t service;
    } val;
  } random_thing_error_t;
  #define RANDOM_THING_ERROR_NETWORK 0
  #define RANDOM_THING_ERROR_SERVICE 1
  #define RANDOM_THING_ERROR_RESPONSE 2
  void random_thing_error_free(random_thing_error_t *ptr);
  typedef struct {
    bool is_err;
    union {
      random_thing_string_t ok;
      random_thing_error_t err;
    } val;
  } random_thing_expected_string_error_t;
  void random_thing_expected_string_error_free(random_thing_expected_string_error_t *ptr);
  void random_thing_get_random_thing(random_thing_request_t *req, random_thing_expected_string_error_t *ret0);
  #ifdef __cplusplus
}
#endif
#endif
