mod nmr_queue;
mod read_conf;

use std::sync::{Arc, Mutex};

use nmr_queue::{NmrQueue, Task};
use worker_queue::TaskQueue;

fn main() {
    let mut queue = NmrQueue::new(Some("conf.txt".to_string()));
    let result = Arc::new(Mutex::new(TaskQueue::new()));
    let task = Task::new("test".to_string(), vec!["{a:1".to_string(), ",b:2}".to_string()], result.clone());
    std::thread::sleep(std::time::Duration::from_secs(2));
    queue.put(task);
    let work = result.lock().expect("Failed to adquire lock").get();
    dbg!();
    let res = work.get_task();
    println!("{}", res);
    result.lock().expect("Failed to adquire lock").task_done(work);
}