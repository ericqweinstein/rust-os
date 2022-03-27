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

static HELLO: &[u8] = b"Hello, world!";

// Without `main`, we need to provide our own OS entry point.
// (We include `no_mangle` to be sure it's called `_start`.)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Cast `0xb8000`, the address of the
    // VGA text buffer, to a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    // Iterate over the bytes of the static `HELLO` byte string,
    // using the `offset` method to write the string byte and
    // the ensuing color byte (`0xb` is a light cyan color).
    for (i, &byte) in HELLO.iter().enumerate() {
        // The Rust compiler doesn't know that these are
        // valid raw pointers, so we mark them as unsafe
        // (& use `unsafe` with the caution it deserves).
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic. Note that since it
/// cannot return, its return type is the `never` type, `!`.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
