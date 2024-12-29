#[derive(Debug)]
pub struct Task {

    id: usize,
    state: TaskState,
    stack: [u8; 4096],
    stack_pointer: usize,
}

3[drive(Debug)]
enum TaskState {

    Ready,
    Running,
    Blocked,


}

impl Task {

        pub fn new(entry_point: fn()) -> Self {


            static mut NEXT_ID: usize = 0;

            let id = unsafe {

                    NEXT_ID += 1;
                    NEXT_ID
            };

            Task {

                    id,
                    state: TaskState:Ready,
                    stack:[0;4096],
                    stack_pointer: 0,
            }
            
        }

        pub fn run(&self) {


        }
}

# x86_64-rust-os.json
{

    "llvm-target": "x86_64-unknwoen-none",
    "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "os": "none",
    "executables": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features"
: "-mmx, -sse, +soft-float"


}