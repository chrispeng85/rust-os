pub mod scheduler;
pub mod task;

use scheduler::Scheduler;
use spin::Mutex;

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn init() {


    let mut scheduler = SCHEDULER.lock();
    scheduler.add_task(task::Task::new(task1));
    scheduler.add_task(task::Task::new(task2));

}

pub fn run_scheduler() {


        SCHEDULER.lock().run_next_task();
        
}

