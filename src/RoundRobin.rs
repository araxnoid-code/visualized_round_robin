use std::{ cell::RefCell, collections::{ VecDeque }, rc::Rc };

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub arrival: i32,
    pub burst_time: i32,
    pub running_time: i32,
}

impl Task {
    pub fn init(name: String, arrival: i32, burst_time: i32) -> Task {
        Self {
            name,
            arrival,
            burst_time,
            running_time: 0,
        }
    }
}

pub struct RoundRobinScheduling {
    pub task_queue: VecDeque<Rc<RefCell<Task>>>,
    pub exec_queue: VecDeque<Rc<RefCell<Task>>>,
    pub running_task: Option<Rc<RefCell<Task>>>,
    pub history: VecDeque<String>,
    pub quantum: i32,
}

impl RoundRobinScheduling {
    pub fn init() -> RoundRobinScheduling {
        Self {
            task_queue: VecDeque::new(),
            exec_queue: VecDeque::new(),
            history: VecDeque::new(),
            running_task: None,
            quantum: 3,
        }
    }

    pub fn spawn_task(&mut self, name: String, arrival: i32, burst_time: i32) {
        let task = Task::init(name, arrival, burst_time);
        self.task_queue.push_back(Rc::new(RefCell::new(task)));
    }

    pub fn running(&mut self) {
        let mut time = 0;
        loop {
            for task in &self.task_queue {
                let arrival_task = task.borrow().arrival;
                if arrival_task == time {
                    self.exec_queue.push_back(task.clone());
                    break;
                }
            }

            let mut billing_habis = false;
            if let Some(running_task) = &self.running_task {
                let mut borrow_running_task = running_task.borrow_mut();
                borrow_running_task.running_time += 1;
                borrow_running_task.burst_time -= 1;

                if
                    borrow_running_task.burst_time <= 0 ||
                    borrow_running_task.running_time >= self.quantum
                {
                    billing_habis = true;
                }
            } else {
                let first_task = self.exec_queue.pop_front().unwrap();
                self.history.push_back(first_task.borrow().name.clone());
                self.running_task = Some(first_task);
            }

            if billing_habis {
                let task = self.running_task.clone().unwrap();
                let task_borrow = &mut task.borrow_mut();

                task_borrow.running_time = 0;
                if task_borrow.burst_time > 0 {
                    self.exec_queue.push_back(task.clone());
                }

                self.running_task = None;
            }

            if let None = self.running_task {
                if self.exec_queue.is_empty() {
                    break;
                }
            }

            time += 1;
        }
    }
}
