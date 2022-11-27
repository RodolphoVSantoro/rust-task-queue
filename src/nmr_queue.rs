use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

use worker_queue::TaskQueue;

use crate::read_conf::read_conf;

#[derive(Clone)]
pub struct Task{
    simulation: String,
    args: Vec<String>,
    result_arc: Arc<TaskQueue<String>>
}

impl Task {
    pub fn new(simulation: String, args: Vec<String>, result_arc: Arc<TaskQueue<String>>) -> Task{
        Task{simulation, args, result_arc}
    }
}

struct NmrQueueData {
    conf_path: Arc<Mutex<Option<String>>>,
    simulation_paths: Arc<Mutex<Option<HashMap<String,String>>>>,
    task_queue: TaskQueue<Task>,
    conf_last_update: Arc<Mutex<Option<u64>>>,
}

pub struct NmrQueue {
    inner: Arc<NmrQueueData>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl NmrQueue {
    pub fn new(conf_path: Option<String>) -> NmrQueue{
        let inner = Arc::new(NmrQueueData::new(conf_path));
        let inner_clone = inner.clone();
        let thread_handle = thread::spawn(move || {
            inner_clone.run();
        });
        NmrQueue{
            inner,
            thread_handle: Some(thread_handle),
        }
    }
    
    pub fn join(self){
        self.thread_handle.expect("Thread was not started").join().expect("Failed to join thread");
    }
    pub fn set_conf_path(&mut self, conf_path: String) {
        *self.inner.conf_path.lock().expect("Failed to adquire lock") = Some(conf_path);
    }

    pub fn put(&mut self, task: Task){
        let conf = self.inner.conf_path.lock().expect("Failed to adquire lock on put");
        if conf.is_none(){
            panic!("No configuration file path set");
        }
        else{
            self.inner.task_queue.put(task);
        }
    }
}

impl NmrQueueData {
    pub fn new(conf_path: Option<String>) -> NmrQueueData{
        NmrQueueData {
            conf_path: Arc::new(Mutex::new(conf_path)),
            simulation_paths: Arc::new(Mutex::new(None)),
            task_queue: TaskQueue::<Task>::new(),
            conf_last_update: Arc::new(Mutex::new(None)),
        }
    }

    fn run(&self){
        loop{
            println!("waiting for simulation");
            
            let work = self.task_queue.get();
            let simulation = work.get_task().simulation;
            let args = work.get_task().args;

            self.update_conf();
            let simulation_paths = self.simulation_paths.lock().expect("Failed to adquire lock").clone().expect("Failed to load simulation paths");
            let command = simulation_paths.get(&simulation).expect(&format!("Failed to get simulation {simulation}"));
            println!("simulating: {simulation}");

            let result = self.run_task(command, &args)
                .expect(&format!("Failed to run {simulation} with args {args:?}"));
            let res_queue = work.get_task().result_arc;
            res_queue.put(result);
            println!("{simulation} done");
            self.task_queue.task_done(work);
        }
    }
    fn run_task(&self, command: &String, args: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let output = Command::new(command)
            .args(args)
            .output()?;
        Ok(String::from_utf8(output.stdout)?)
    }

    fn was_updated(&self) -> bool {
        true
        //todo("check if configuration file was updated");
    }

    fn update_conf(&self) {
        let conf = self.conf_path.lock().expect("Failed to adquire lock").clone().expect("configuration path not set");
        if self.was_updated(){
            *self.simulation_paths.lock().expect("Failed to adquire lock") = Some(
                read_conf(
                    &conf
                )
                .expect("Error reading configuration file")
            );
        }
    }
}
