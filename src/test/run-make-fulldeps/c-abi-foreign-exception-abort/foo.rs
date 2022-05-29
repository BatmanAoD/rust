// Tests that `"C-unwind"` with `panic=abort` aborts the process rather than letting a C++
// exception propagate.

#![feature(c_unwind)]

use std::panic::catch_unwind;

extern "C-unwind" {
    fn test_cxx_exception();
}

fn main() {
    let caught_unwind = catch_unwind(|| {
        unsafe { test_cxx_exception() }
        unreachable!("C++ function should have thrown instead of returned");
    });
    unreachable!("catch_unwind should not be able to catch C++ exception");
}
