#include <stdio.h>


//dirty to_lower
void fn_pointer(char *data) {
    puts("fn_pointer_begin");
    for(size_t i=0; i < 10; ++i) {
        putchar(data[i]);
        data[i] += 32;
    }
    putchar('\n');
    puts("fn_pointer_end");
}

//dirty to_upper
void fn_fixedarray(char data[10]) {
    puts("fn_fixedarray_begin");
    for(size_t i=0; i < 10; ++i) {
        putchar(data[i]);
        data[i] -= 32;
    }
    putchar('\n');
    puts("fn_fixedarray_end");
}
