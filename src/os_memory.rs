use core::fmt::Display;
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        OffsetPageTable,
        FrameAllocator,
        PageTable,
        Size4KiB,
    },
    PhysAddr,
    VirtualAddr as VirtualAddr,
};
use x86_64::structures::paging::{PhysFrame, Translate};

// 1。 内存预留
// 2。 out of 检查
// 3。 引导程序 （不同规格的设备差别） PIC
// load image
//
pub unsafe fn init(physical_memory_offset: VirtualAddr) -> OffsetPageTable {
    OffsetPageTable::new(active_table(physical_memory_offset), physical_memory_offset)
}

unsafe fn active_table(physical_memory_offset: VirtualAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    // 来自 lib 实现 不关心内部
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let page_table_ptr: *mut PageTable = physical_memory_offset + phys.as_u64().as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {




    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootOs {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootOs {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootOs {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item=PhysFrame> {
        let usable_regions = self.memory_map.iter().filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootOs {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
