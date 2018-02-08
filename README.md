# nvml

[nvml] is a rust crate providing mid-level bindings for [PMDK](http://pmem.io), the persistent memory libraries (formerly NVML) using the [nvml-sys](https://github.com/lemonrock/nvml-sys) FFI bindings.

It currently requires rust nightly to compile.


## Building for the Intel Ivy Bridge CPU

[nvml] has optimizations to use Intel's `RDRAND` instructions (confusingly known as `rdrnd` to LLVM). To build with them, from the root of the source repository do once:-

```bash
rustup component add rust-src
cargo install xargo
```

Then, for each build, from the root of the source repository do:-

```bash
(cd workspace/nvml; RUST_TARGET_PATH="$(pwd)" xargo build --target x86_64-apple-darwin-ivybridge)
```

### Gotchas

* Sadly, there's a bug somewhere in the combination of `xargo`, `cargo` and `rustc`, such that the `CARGO_TARGET_DIR='target'` setting is not honoured after compiling the sysroot. This means that a `.cargo/config` file must not change the target directory.
* Also, there's a bug in `rustc` such that `RUSTFLAGS='-C target-feature=rdrnd'` will cause `#[cfg(target_feature = "rdrnd")]"` to *not* work.


## Licensing

The license for this project is MIT.

[nvml]: https://github.com/lemonrock/nvml "nvml GitHub page"
