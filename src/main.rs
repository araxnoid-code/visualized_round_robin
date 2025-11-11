use std::{ cell::RefCell, collections::{ VecDeque, vec_deque }, rc::Rc };

#[derive(Debug)]
struct Task {
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

struct TaskQueue {
    pub task_queue: VecDeque<Rc<RefCell<Task>>>,
    pub exec_queue: VecDeque<Rc<RefCell<Task>>>,
    pub running_task: Option<Rc<RefCell<Task>>>,
    pub history: VecDeque<String>,
    pub quantum: i32,
}

impl TaskQueue {
    pub fn init() -> TaskQueue {
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
}

fn main() {
    let mut task_queue = TaskQueue::init();
    task_queue.spawn_task("p1".to_string(), 0, 4);
    task_queue.spawn_task("p2".to_string(), 1, 3);
    task_queue.spawn_task("p3".to_string(), 2, 8);
    task_queue.spawn_task("p4".to_string(), 3, 6);
    task_queue.spawn_task("p5".to_string(), 4, 2);
    task_queue.spawn_task("p6".to_string(), 6, 5);
    task_queue.spawn_task("p7".to_string(), 7, 4);
    task_queue.spawn_task("p8".to_string(), 8, 3);

    // visialized
    let mut time = 0;
    loop {
        for task in &task_queue.task_queue {
            let arrival_task = task.borrow().arrival;
            if arrival_task == time {
                task_queue.exec_queue.push_back(task.clone());
                break;
            }
        }

        let mut billing_habis = false;
        if let Some(running_task) = &task_queue.running_task {
            let mut borrow_running_task = running_task.borrow_mut();
            borrow_running_task.running_time += 1;
            borrow_running_task.burst_time -= 1;

            if
                borrow_running_task.burst_time <= 0 ||
                borrow_running_task.running_time >= task_queue.quantum
            {
                billing_habis = true;
            }
        } else {
            let first_task = task_queue.exec_queue.pop_front().unwrap();
            task_queue.history.push_back(first_task.borrow().name.clone());
            task_queue.running_task = Some(first_task);
        }

        if billing_habis {
            let task = task_queue.running_task.unwrap();
            let task_borrow = &mut task.borrow_mut();

            task_borrow.running_time = 0;
            if task_borrow.burst_time > 0 {
                task_queue.exec_queue.push_back(task.clone());
            }

            task_queue.running_task = None;
        }

        if let None = task_queue.running_task {
            if task_queue.exec_queue.is_empty() {
                break;
            }
        }

        time += 1;
    }

    println!("{:?}", task_queue.history)
}
