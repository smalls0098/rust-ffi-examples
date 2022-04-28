#include <stdint.h>

const char* get_version();

typedef struct {
    const char* token;
    const char* core;
} TokenInfo;

TokenInfo get_token(const char* token);

// 销毁字符串
void rust_free(char *);
