use x86_64::{

    structures::paging::{PhysFrame, Size4KiB},
    PhysAddr,

};

pub struct BootInfoFrameAllocator {

    memory_map: &'static MemoryMap,
    next: usize,

}


impl BootInfoFrameAllocator {

    pub fn init() -> Self {

        BootInfoFrameAllocator {

                memory_map: unsafe {&*(0x500 as *const MemoryMap)},
                next: 0,
        }

    }

    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {


            let frame = self.memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            .flat_map(|r| r.range())
            .nth(self.next);

            self.next += 1;
            frame
    }



}


