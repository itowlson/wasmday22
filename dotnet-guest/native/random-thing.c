#include <stdlib.h>
#include "random-thing.h"

__attribute__((weak, export_name("canonical_abi_realloc")))
void *canonical_abi_realloc(
void *ptr,
size_t orig_size,
size_t org_align,
size_t new_size
) {
  void *ret = realloc(ptr, new_size);
  if (!ret)
  abort();
  return ret;
}

__attribute__((weak, export_name("canonical_abi_free")))
void canonical_abi_free(
void *ptr,
size_t size,
size_t align
) {
  free(ptr);
}
#include <string.h>

void random_thing_string_set(random_thing_string_t *ret, const char *s) {
  ret->ptr = (char*) s;
  ret->len = strlen(s);
}

void random_thing_string_dup(random_thing_string_t *ret, const char *s) {
  ret->len = strlen(s);
  ret->ptr = canonical_abi_realloc(NULL, 0, 1, ret->len);
  memcpy(ret->ptr, s, ret->len);
}

void random_thing_string_free(random_thing_string_t *ret) {
  canonical_abi_free(ret->ptr, ret->len, 1);
  ret->ptr = NULL;
  ret->len = 0;
}
void random_thing_error_free(random_thing_error_t *ptr) {
  switch ((int32_t) ptr->tag) {
    case 0: {
      random_thing_string_free(&ptr->val.network);
      break;
    }
  }
}
void random_thing_expected_string_error_free(random_thing_expected_string_error_t *ptr) {
  if (!ptr->is_err) {
    random_thing_string_free(&ptr->val.ok);
  } else {
    random_thing_error_free(&ptr->val.err);
  }
}

__attribute__((aligned(4)))
static uint8_t RET_AREA[16];
__attribute__((import_module("random-thing"), import_name("get-random-thing")))
void __wasm_import_random_thing_get_random_thing(int32_t, int32_t, int32_t);
void random_thing_get_random_thing(random_thing_request_t *req, random_thing_expected_string_error_t *ret0) {
  int32_t variant;
  int32_t variant1;
  switch ((int32_t) (*req).tag) {
    case 0: {
      variant = 0;
      variant1 = 0;
      break;
    }
    case 1: {
      const random_thing_animal_type_t *payload0 = &(*req).val.animal_fact;
      variant = 1;
      variant1 = (int32_t) *payload0;
      break;
    }
  }
  int32_t ptr = (int32_t) &RET_AREA;
  __wasm_import_random_thing_get_random_thing(variant, variant1, ptr);
  random_thing_expected_string_error_t expected;
  switch ((int32_t) (*((uint8_t*) (ptr + 0)))) {
    case 0: {
      expected.is_err = false;
      
      expected.val.ok = (random_thing_string_t) { (char*)(*((int32_t*) (ptr + 4))), (size_t)(*((int32_t*) (ptr + 8))) };
      break;
    }
    case 1: {
      expected.is_err = true;
      random_thing_error_t variant2;
      variant2.tag = (int32_t) (*((uint8_t*) (ptr + 4)));
      switch ((int32_t) variant2.tag) {
        case 0: {
          variant2.val.network = (random_thing_string_t) { (char*)(*((int32_t*) (ptr + 8))), (size_t)(*((int32_t*) (ptr + 12))) };
          break;
        }
        case 1: {
          variant2.val.service = (uint16_t) ((int32_t) (*((uint16_t*) (ptr + 8))));
          break;
        }
        case 2: {
          break;
        }
      }
      
      expected.val.err = variant2;
      break;
    }
  }*ret0 = expected;
}
