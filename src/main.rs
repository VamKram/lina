#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use rust_64_bit_os::println;


// 引导进内核
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_64_bit_os::{os_allocator, os_memory::{self, BootOs, init}};
    use x86_64::VirtAddr;

    println!("Welcome to lina 🎉");

    rust_64_bit_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootOs::init(&boot_info.memory_map) };
    // 创建堆内存 模拟
    os_allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Failed");
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    Rc::new(vec![1, 2, 3]).clone();
    // 主动 drop
    // todo 并发
    core::mem::drop(reference_counted);
    rust_64_bit_os::hlt();
}

// 重写 panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_64_bit_os::hlt();
}
