#include "stdint.h"

typedef struct {
  int len;
  uint8_t* data;
} Result;

typedef struct {
  void (*register_op)(char*, Result (*f)(uint8_t*))
} Registrar;

Result cSync(uint8_t* data) {
  Result result = { 0, data };
  return result;
}

void register_ops(Registrar* registrar) {
  registrar->register_op("cSync", cSync);
}
