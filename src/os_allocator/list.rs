use alloc::alloc::{GlobalAlloc, Layout};
use crate::os_allocator::{align_up, Locked};
use core::{mem, ptr};

struct ListNode {
    size: usize,
    // 全生命周期
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    // 取到自身
    fn initial_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.initial_addr() + self.size
    }
}

pub struct ListAllocator {
    head: ListNode,
}

impl ListAllocator {
    /// TODO 重新创建.
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    /// 初始化unsafe
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free(heap_start, heap_size);
    }

    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        if size > 16 * 1024 * 1024 {
            Err("out of memory")
        }
        (size, layout.align())
    }

    unsafe fn add_free(&mut self, addr: usize, size: usize) {
        let node_ptr = self.gen_list(addr, size);
        self.head.next = Some(&mut *node_ptr)
    }

    unsafe fn gen_list(&mut self, addr: usize, size: usize) -> *mut ListNode {
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        node_ptr
    }

    fn find_loc(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;
        let err_call_back = || {
            current = current.next.as_mut().unwrap();
        };
        while let Some(ref mut region) = current.next {
            Self::alloc_from_region(&region, size, align).unwrap_or_else(err_call_back);
            let next = region.next.take();
            let ret = Some((current.next.take().unwrap(), alloc_start));
            current.next = next;
            return ret;
        }
        None
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.initial_addr(), align);
        let alloc_end = alloc_start.checked_add(size).unwrap().expect("");
        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            return Err(());
        }
        Ok(alloc_start)
    }
}

unsafe impl GlobalAlloc for Locked<ListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = ListAllocator::size_align(layout);
        let mut allocator = self.lock();
        let err_callback = || ptr::null_mut();
        let (region, alloc_start) = allocator.find_loc(size, align).unwrap_or_else(err_callback)
        let alloc_end = alloc_start.checked_add(size).expect("overflow");
        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 {
            allocator.add_free(alloc_end, excess_size);
        }
        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // perform layout adjustments
        let (size, _) = ListAllocator::size_align(layout);

        self.lock().add_free(ptr as usize, size)
    }
}
