use crate::{os_gdt, hlt, print, println};
use pic8259::ChainedPics;
use spin;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// zak 的 pure os mark 记录 pic tinjiao
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    // 标准实现
    static ref InterruptDescTable: InterruptDescriptorTable = {
        let mut interrupt_desc_table = InterruptDescriptorTable::new();
        interrupt_desc_table.breakpoint.set_handler_fn(breakpoint_handler);
        interrupt_desc_table.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            interrupt_desc_table.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(os_gdt::DOUBLE_FAULT_IST_INDEX);
             interrupt_desc_table[InterruptIndex::Timer.as_usize()].set_handler_fn(handle_timer);
             interrupt_desc_table[InterruptIndex::Keyboard.as_usize()].set_handler_fn(handler_keyboard);
        }
        interrupt_desc_table
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Error ->", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    println!("Error Code: {:?}", error_code);
    hlt();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> () {
    println("{:#?}", stack_frame);
}


// -------


extern "x86-interrupt" fn handle_timer(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn handler_keyboard(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0x60);

    lazy_static! {
        // 出实话
        // 这里先共享
        // 来自 pc keyboard
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        let key =   keyboard.process_keyevent(key_event).unwrap();
        match  keyboard.process_keyevent(key_event) {
            Some(DecodedKey::Unicode(character)) => print!("{}", character),
            Some(DecodedKey::RawKey(key)) => print!("{}", character),
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
