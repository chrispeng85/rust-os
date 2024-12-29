pub mod frame_allocator;

pub mod paging;

use x86_64::{

    structures::paging::{


        PageTable,
        OffsetPageTable,
        PhysFrame,
        Page,
        Mapper,
        Size4KiB,
    },

    VirtAddr,
    PhysAddr,

};

pub fn init() {

    let mut frame_allocator = frame_allocator::BootInfoFrameAllocator::init();
    let mut page_table = unsafe { paging::init(VirtAddr::new(0xffff_ffff_8000_0000))};

    paging::map_kernel(&mut page_table, &mut frame_allocator);

    paging::init_heap(&mut page_table, &mut frame_allocator)
    .expect("heap initialization failed");

}



