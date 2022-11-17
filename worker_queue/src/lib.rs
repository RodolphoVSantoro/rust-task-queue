use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use queues::{IsQueue, Queue};

pub struct Work<T>{
    task: T,
    working: Arc<Mutex<bool>>
}

impl<T> Work<T> where T: Clone{
    fn new(task: T, working: Arc<Mutex<bool>>) -> Work<T>{
        Work{task, working}
    }

    pub fn get_task(&self) -> T {
        self.task.clone()
    }
}

pub struct TaskQueue<T> where T: Clone{
    queue: Mutex<Queue<T>>,
    working: Arc<Mutex<bool>>,
    has_work: Mutex<bool>
}

impl<T> Default for TaskQueue<T> where T: Clone {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TaskQueue<T> where T: Clone{
    #[must_use = "unused created object"]
    pub fn new() -> TaskQueue<T>{
        TaskQueue{
            queue: Mutex::new(Queue::<T>::new()),
            working: Arc::new(Mutex::new(false)),
            has_work: Mutex::new(false)
        }
    }

    pub fn get(&mut self) -> Work<T>{
        loop{
            let has_work = self.has_work.lock()
                .expect("Failed to adquire lock to check if queue has work");
            let is_working = self.working.lock()
                .expect("Failed to adquire lock to check if worker is working");
            if *has_work && !*is_working{
                break;
            }
            drop(has_work);
            drop(is_working);
            thread::sleep(Duration::from_secs(1));
        }
        let task_queue_lock = self.queue.lock();
        let task = task_queue_lock.expect("Failed adquire lock to get task")
            .peek().expect("Failed to peek queue");
        let mut working = self.working.lock().expect("Failed to adquire lock to check if worker as working");
        *working = true;
        return Work::new(task, Arc::clone(&self.working));
    }

    pub fn put(&mut self, task: T){
        let mut has_work = self.has_work.lock().expect("Failed to adquire lock to check if queue has work");
        let mut queue = self.queue.lock().expect("Failed to adquire lock to put task");
        queue.add(task).expect("Failed to add task to queue");
        *has_work = true;
    }

    pub fn task_done(&mut self, work: Work<T>){
        let mut queue_lock = self.queue.lock().expect("Failed to adquire lock to remove task");
        queue_lock.remove().expect("Failed to remove task from queue");
        if queue_lock.size() == 0{
            let mut has_work = self.has_work.lock().expect("Failed to adquire lock to check if queue has work");
            *has_work = false;
        }
        let mut working = work.working.lock().expect("Failed to adquire lock to check if worker is working");
        *working = false;
    }

    pub fn task_incomplete(&mut self, work: Work<T>){
        let mut working = work.working.lock().expect("Failed to adquire lock to check if worker is working");
        *working = false;
    }
}

    
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Clone)]
    struct TestTask{
        name: String
    }

    impl TestTask {
        fn new(name: &str) -> TestTask{
            TestTask{name: name.to_string()}
        }
    }
    #[test]
    fn test_task_queue(){
        let task0 = TestTask::new("t1");
        let task1 = TestTask::new("t2");
        let task2 = TestTask::new("t3");

        let mut task_queue = TaskQueue::new();
        
        task_queue.put(task0);
        let work = task_queue.get();
        task_queue.task_incomplete(work);
        
        let work2 = task_queue.get();
        let task = work2.get_task();
        assert_eq!(task.name, "t1");
        
        task_queue.put(task1);
        task_queue.task_done(work2);
        task_queue.put(task2);
        
        let work3 = task_queue.get();
        let task = work3.get_task();
        assert_eq!(task.name, "t2");
        task_queue.task_done(work3);

        let work4 = task_queue.get();
        let task = work4.task.clone();
        assert_eq!(task.name, "t3");
        task_queue.task_done(work4);
    }
}
