#include "stdint.h"


int32_t add(int32_t a, int32_t b) {
    return a + b;
}

// gcc - fPIC -shared -o libfoo.so foo.c
