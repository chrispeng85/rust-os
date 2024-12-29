use super::task::Task;
use alloc::collections::VecDeque;

pub struct Scheduler {

        tasks: VecDeque<Task>,
} 

impl Scheduler {

    pub const fn new() -> Self {

            Scheduler {

                tasks: VecDeque::new(),
            }
    }


    pub fn add_task(&mut self, task: Task) {

        self.tasks.push_back(task);
    }

    pub fn run_next_task(&mut self) {

        if let Some(task) = self.tasks.pop_front() {

                task.run();
                self.tasks.push_back(task);
        }
    }
}




