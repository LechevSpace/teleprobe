# teleprobe-meta

This crate allows embedding metadata into ELF binaries so that [teleprobe](https://github.com/embassy-rs/teleprobe)
can autodetect it. This way you can run the tests by simply doing `teleprobe client run <ELF>`, without
adding any extra flags.

## Usage

First, include the `teleprobe.x` linker script. Either via `build.rs` (recommended)

```rust
println!("cargo:rustc-link-arg-bins=-Tteleprobe.x");
```

or in `.cargo/config.toml` (older way, not recommended)

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
rustflags = [
  "-C", "link-arg=-Tteleprobe.x",
]
```

Then, you can specify metadata, for example:

```rust
teleprobe_meta::target!(b"rpi-pico");
```

## Minimum supported Rust version (MSRV)

`teleprobe-meta` is guaranteed to compile on the latest stable Rust version at the time of release. It might compile with older versions but that may change in any new patch release.

## License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
