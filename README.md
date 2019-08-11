# AES

128 bit AES implementation in Rust I have tried to keep simple.

## Build:

Build with:

```bash
cargo build --release
```

Then in `./target/release/` you will find the executable `aese`. Move this to `PATH` if you want to be able to use it anywhere.

## Usage:

```bash
aese [encrypt/decrypt] <path> -o <output path> -k "key"
```

View `--help` for more information.
