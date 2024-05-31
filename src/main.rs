use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use nix::{
    fcntl, 
    fcntl::OFlag,
    sys::stat::Mode,
};

const KVM_DEVICE: &str = "/dev/kvm";

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
}
