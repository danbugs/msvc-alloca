#include <stddef.h>

void* _alloca_wrapper(size_t size) {
    int vla[size];

    // initialize w/ 0
    for (int i = 0; i < size; i++) {
        vla[i] = 0;
    }
}
