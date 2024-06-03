//
// Port of kvm-hello-world.c to Rust
//
// The C version exercises the KVM API by creating a VM, assigning it some
// memory and VCPUs and starting it in one of the x86-64 processor modes.
//
// Author: Tim Potter <tpot@frungy.org>
//

// File descriptor creation and manipulation
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::num::NonZeroUsize;

// Rust-friendly bindings to various system functions
use nix::{
    fcntl,
    fcntl::OFlag,
    sys::stat::Mode,
    ioctl_none_bad,
    ioctl_write_ptr,
    request_code_none,
    sys::{mman, mman::MapFlags, mman::ProtFlags},
};

// FFI bindings autogenerated from linux/kvm.h
use kvm_bindings::{
    KVMIO,
    KVM_API_VERSION,
    kvm_userspace_memory_region,
    kvm_run,
};

const KVM_DEVICE: &str = "/dev/kvm";
const MAP_SIZE: usize = 0x1000;

// Unfortunately the kvm_bindings crate does not export ioctl sequence numbers
// so we must hardcode them and use "bad" ioctls.
ioctl_none_bad!(kvm_get_api_version,    request_code_none!(KVMIO, 0x00));
ioctl_none_bad!(kvm_create_vm,          request_code_none!(KVMIO, 0x01));
ioctl_none_bad!(kvm_create_vcpu,        request_code_none!(KVMIO, 0x41));
ioctl_none_bad!(kvm_get_vcpu_mmap_size, request_code_none!(KVMIO, 0x04));

ioctl_write_ptr!(kvm_set_user_memory_region, KVMIO, 0x46, kvm_userspace_memory_region);

struct Vm {
    sys_fd: OwnedFd,
    vm_fd: OwnedFd,
    mem: u64,
}

impl Vm {

    // Create new VM
    pub fn new() -> Result<Self, nix::Error> {

        // Open /dev/kvm
        let sys_fd: OwnedFd = match fcntl::open(KVM_DEVICE, OFlag::O_RDWR, Mode::empty()) {
            Ok(fd) => unsafe {
                assert!(fd != -1);
                FromRawFd::from_raw_fd(fd)
            },
            Err(errno) => return Err(errno),
        };

        println!("sys_fd = {0}", AsRawFd::as_raw_fd(&sys_fd));

        // Get KVM API version
        let api_ver = match unsafe {
            kvm_get_api_version(AsRawFd::as_raw_fd(&sys_fd))
        } {
            Ok(result) => {
                assert!(result == KVM_API_VERSION as i32, "Unknown KVM API version: {result}");
                result
            },
            Err(errno) => return Err(errno),
        };

        println!("KVM API version = {api_ver}");

        // Create a VM
        let vm_fd: OwnedFd = match unsafe {
            kvm_create_vm(AsRawFd::as_raw_fd(&sys_fd))
        } {
            Ok(fd) => unsafe {
                assert!(fd != -1);
                FromRawFd::from_raw_fd(fd)
            },
            Err(errno) => return Err(errno),
        };

        println!("vm_fd = {0}", AsRawFd::as_raw_fd(&vm_fd));

        // Create and attach memory
        let mem = match unsafe {
            mman::mmap_anonymous(
                None,
                NonZeroUsize::new(MAP_SIZE).expect("User memory size is zero"),
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_ANONYMOUS | MapFlags::MAP_SHARED,
            )
        } {
            Ok(result) => {
                result.as_ptr() as u64
            },
            Err(errno) => return Err(errno),
        };

        match unsafe {
            kvm_set_user_memory_region(
                AsRawFd::as_raw_fd(&vm_fd),
                &kvm_userspace_memory_region {
                    slot: 0,
                    flags: 0,
                    guest_phys_addr: 0,
                    memory_size: MAP_SIZE as u64,
                    userspace_addr: mem,
                },
            )
        } {
            Ok(_) => {},
            Err(errno) => return Err(errno),
        };

        Ok(Self{
            sys_fd,
            vm_fd,
            mem,
        })
    }
}

fn main() {

    // Initialise VM
    let vm = Vm::new().expect("Unable to initialise VM");

    // Create a vCPU for the VM
    let kvm_vcpu_fd: OwnedFd = match unsafe {
        kvm_create_vcpu(AsRawFd::as_raw_fd(&vm.vm_fd))
    } {
        Ok(fd) => unsafe {
            assert!(fd != -1);
            FromRawFd::from_raw_fd(fd)
        },
        Err(errno) => {
            eprintln!("Error creating VCPU: {errno}");
            std::process::exit(1);
        },
    };

    println!("kvm_vcpu_fd = {0}", AsRawFd::as_raw_fd(&kvm_vcpu_fd));

    // Create kvm_run structure
    let vcpu_mmap_size: NonZeroUsize = match unsafe {
        kvm_get_vcpu_mmap_size(AsRawFd::as_raw_fd(&vm.sys_fd))
    } {
        Ok(result) => {
            NonZeroUsize::new(result as usize).expect("mmap_size is zero")
        },
        Err(errno) => {
            eprintln!("Error getting VCPU mmap() size: {errno}");
            std::process::exit(1);
        },
    };

    println!("vcpu_mmap_size = {vcpu_mmap_size}");

    let _kvm_run: *mut kvm_run = match unsafe {
        mman::mmap(
            None,
            vcpu_mmap_size,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            kvm_vcpu_fd,
            0,
        )
    } {
        Ok(result) => {
            result.as_ptr() as *mut kvm_run
        },
        Err(errno) => {
            eprintln!("Error calling mmap(): {errno}");
            std::process::exit(1);
        },
    };
}
