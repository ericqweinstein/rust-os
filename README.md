An OS in Rust
=============

Following [this tutorial](https://os.phil-opp.com/).

## Building
You can build the executable via `cargo build --target thumbv7em-none-eabihf`. [Alternately](https://os.phil-opp.com/freestanding-rust-binary/), you can build for each host system like so:

```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles

# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```
