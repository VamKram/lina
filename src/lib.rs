extern crate alloc;
extern crate core;

pub mod os_allocator;
pub mod os_gdt;
pub mod interrupts;
pub mod os_memory;
pub mod vga_buffer;

pub fn init() {
    os_gdt::init();
    interrupts::init_idt();
    // pic 内部方法
    unsafe { interrupts::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();
}

pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
    panic!("no way")
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Error -> {:?}", layout)
}
