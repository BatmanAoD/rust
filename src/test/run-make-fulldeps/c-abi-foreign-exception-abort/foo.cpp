#include <cstdio>

void println(const char* s) {
    std::puts(s);
    std::fflush(stdout);
}

struct exception {};

extern "C" {
    void throw_cxx_exception() {
        println("throwing C++ exception");
        throw exception();
    }
}
