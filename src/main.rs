mod nmr_queue;
mod read_conf;

use std::sync::Arc;

use nmr_queue::{NmrQueue, Task};
use worker_queue::TaskQueue;

fn main() {
    let mut queue = NmrQueue::new(Some("conf.txt".to_string()));
    
    let result = Arc::new(TaskQueue::new());
    let task = Task::new("test".to_string(), vec!["{a:1".to_string(), ",b:2}".to_string()], result.clone());

    let result2 = Arc::new(TaskQueue::new());
    let task2 = Task::new("test".to_string(), vec!["{a:4".to_string(), ",b:5}".to_string()], result2.clone());

    std::thread::sleep(std::time::Duration::from_secs(1));
    queue.put(task);    
    std::thread::sleep(std::time::Duration::from_millis(300));
    queue.put(task2);

    let work = result.get();
    let res = work.get_task();
    println!("res: {}", res);
    result.task_done(work);
    
    let work2 = result2.get();
    let res2 = work2.get_task();
    println!("res: {}", res2);
    result2.task_done(work2);
}
