use x86_64::{

    structures::paging::{

        page_table, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB

    }, PhysAddr, VirtAddr

};

use super::frame_allocator;

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {

        let level_4_table = active_level_4_table(physcial_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
}

pub fn map_kernel (

    page_table: &mut impl Mapper<size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,

)  {
    
} 