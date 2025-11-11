use RoundRobin::*;

fn main() {
    let mut task_queue = RoundRobinScheduling::init();
    task_queue.spawn_task("p1".to_string(), 0, 4);
    task_queue.spawn_task("p2".to_string(), 1, 3);
    task_queue.spawn_task("p3".to_string(), 2, 8);
    task_queue.spawn_task("p4".to_string(), 3, 6);
    task_queue.spawn_task("p5".to_string(), 4, 2);
    task_queue.spawn_task("p6".to_string(), 6, 5);
    task_queue.spawn_task("p7".to_string(), 7, 4);
    task_queue.spawn_task("p8".to_string(), 8, 3);

    task_queue.running();

    println!("{:?}", task_queue.history)
}
