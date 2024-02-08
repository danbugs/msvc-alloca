#include <stddef.h>

void* _alloca_wrapper(size_t size) {
    return _alloca(size);
}
