use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// 标准实现
// pure os
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4 * 1024;
            #[repr(align(16))]
            struct Stack([u8; STACK_SIZE]);
           ( VirtAddr::from_ptr(unsafe { &(Stack([0; STACK_SIZE])) }) + STACK_SIZE)
        };
        tss
    };
}

lazy_static! {
    // gdt 初始化失败还不确定怎么搞。。
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_manager = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_manager = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_manager,
                tss_manager,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;

    use x86_64::instructions::segmentation::set_cs;
    // 同上 todo
    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
