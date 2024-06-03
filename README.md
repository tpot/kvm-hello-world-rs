# kvm-hello-world-rs

Port of
[kvm-hello-world.c](https://github.com/dpw/kvm-hello-world/blob/master/kvm-hello-world.c)
to Rust.

## Resources

* [Using the KVM API](https://lwn.net/Articles/658511/)
* Crates
  * [libc](https://docs.rs/libc/0.2.155/libc/index.html) - FFI bindings to system libc
  * [nix](https://docs.rs/nix/latest/nix/index.html) - Rust-friendly bindings to libc and other system functions
  * [kvm-bindings](https://docs.rs/kvm-bindings/latest/kvm_bindings/index.html) - automatically generated FFI bindings for the KVM API from `linux/kvm.h` using [bindgen](https://crates.io/crates/bindgen)
