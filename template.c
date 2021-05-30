#include "walloc.c"

#define WASM_EXPORT(name) \
  __attribute__((export_name(#name))) \
  name

void* WASM_EXPORT(IPDABI_STEP)(unsigned char* histMine, unsigned char* histTheirs, unsigned int len, void* memory, double randomness) {
    int move = 0; // replace with good alg; coop is 1
    
    // housekeeping stuff below
    void* retval = malloc(sizeof(*void) + 1);
    *retval = memory;
    *(retval + sizeof(*void)) = move;
    return retval;
}

void* WASM_EXPORT(IPDABI_MALLOC)(size_t size) {
    return malloc(size);
}

void WASM_EXPORT(IPDABI_FREE)(void* ptr) {
    free(ptr);
}
