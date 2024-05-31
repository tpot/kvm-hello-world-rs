use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

use nix::{
    fcntl,
    fcntl::OFlag,
    sys::stat::Mode,
    ioctl_none_bad,
    request_code_none,
};

use kvm_bindings::{
    KVMIO,
    KVM_API_VERSION,
};

const KVM_DEVICE: &str = "/dev/kvm";

// Unfortunately the kvm_bindings crate does not export ioctl sequence numbers
// so we must hardcode them and use "bad" ioctls.
ioctl_none_bad!(kvm_get_api_version, request_code_none!(KVMIO, 0x00));
ioctl_none_bad!(kvm_create_vm,       request_code_none!(KVMIO, 0x01));

fn main() {

    // Open /dev/kvm
    let kvm_fd: OwnedFd = match fcntl::open(KVM_DEVICE, OFlag::O_RDWR, Mode::empty()) {
        Ok(fd) => unsafe {
            assert!(fd != -1);
            FromRawFd::from_raw_fd(fd)
        },
        Err(errno) => {
            eprintln!("Error opening {KVM_DEVICE}: {errno}");
            std::process::exit(1);
        },
    };

    println!("kvm_fd = {0}", AsRawFd::as_raw_fd(&kvm_fd));

    // Get KVM API version
    let api_ver = match unsafe {
        kvm_get_api_version(kvm_fd.as_raw_fd())
    } {
        Ok(result) => {
            assert!(result == KVM_API_VERSION as i32, "Unknown KVM API version: {result}");
            result
        },
        Err(errno) => {
            eprintln!("Error in kvm_create_vm: {errno}");
            std::process::exit(1);
        },
    };

    println!("KVM API version = {api_ver}");

    // Create a VM
    let kvm_vm_fd: OwnedFd = match unsafe {
        kvm_create_vm(kvm_fd.as_raw_fd())
    } {
        Ok(fd) => unsafe {
            assert!(fd != -1);
            FromRawFd::from_raw_fd(fd)
        },
        Err(errno) => {
            eprintln!("Error creating VM: {errno}");
            std::process::exit(1);
        },
    };

    println!("kvm_vm_fd = {0}", AsRawFd::as_raw_fd(&kvm_vm_fd));
}
