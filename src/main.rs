//! An operating system in Rust!
//! See: https://os.phil-opp.com/

// Since we're building an operating system, we can't rely on the standard
// library. Using `[no_std]` here prevents linking of the standard library
// and results in a "freestanding" (bare metal) executable (though we still
// get language features like iterators, closures, the ownership system, &c).
#![no_std]
// We also don't have access to `crt0` ("C runtime zero"), which starts
// a minimal C runtime that in turn invokes the Rust runtime entry point.
#![no_main]

// Without the standard library, we don't have a default panic handler.
use core::panic::PanicInfo;

// Without `main`, we need to provide our own OS entry point.
// (We include `no_mangle` to be sure it's called `_start`.)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic. Note that since it
/// cannot return, its return type is the `never` type, `!`.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
