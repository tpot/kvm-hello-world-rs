# kvm-hello-world-rs

Port of
[kvm-hello-world.c](https://github.com/dpw/kvm-hello-world/blob/master/kvm-hello-world.c)
to Rust. It turns out that the C version didn't load any code so I modified my
version to run some sample code from [Using the KVM
API](https://lwn.net/Articles/658511/) article on lwn.net.

## Usage

```
$ cargo run
   Compiling kvm-hello-world-rs v0.1.0 (/home/tpot/repos/kvm-hello-world-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/kvm-hello-world-rs`
sys_fd = 3
KVM API version = 12
vm_fd = 4
kvm_vcpu_fd = 5
vcpu_mmap_size = 12288
I/O dir=1 port=0x3f8 size=1 count=1
I/O dir=1 port=0x3f8 size=1 count=1
Program halted
```

## Resources

* [Using the KVM API](https://lwn.net/Articles/658511/)
* Crates
  * [libc](https://docs.rs/libc/0.2.155/libc/index.html) - FFI bindings to system libc
  * [nix](https://docs.rs/nix/latest/nix/index.html) - Rust-friendly bindings to libc and other system functions
  * [kvm-bindings](https://docs.rs/kvm-bindings/latest/kvm_bindings/index.html) - automatically generated FFI bindings for the KVM API from `linux/kvm.h` using [bindgen](https://crates.io/crates/bindgen)
