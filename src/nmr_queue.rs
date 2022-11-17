use worker_queue::TaskQueue;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;


#[derive(Clone)]
pub struct Task{
    simulation: String,
    args: Vec<String>,
    result_arc: Arc<Mutex<String>>
}

struct NmrQueueData {
    conf_path: Option<String>,
    simulation_paths: Option<HashMap<String,String>>,
    task_queue: TaskQueue<Task>,
    conf_last_update: Option<u64>,
}

pub struct NmrQueue {
    inner: Arc<Mutex<NmrQueueData>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl NmrQueue {
    pub fn new() -> NmrQueue{
        let inner = Arc::new(Mutex::new(NmrQueueData::new(None)));
        let inner_clone = inner.clone();
        let thread_handle = thread::spawn(move || {
            let mut inner = inner_clone.lock().expect("Failed to create NmrQueueData");
            inner.run();
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
        self.inner.lock().expect("Failed to adquire lock on set conf").conf_path = Some(conf_path);
    }

    pub fn put(&mut self, task: Task){
        if self.inner.lock().expect("Failed to adquire lock on put").conf_path.is_none(){
            panic!("No configuration file path set");
        }
        else{
            self.inner.lock().expect("Failed to adquire lock").task_queue.put(task);
        }
    }
}

impl NmrQueueData {
    pub fn new(conf_path: Option<String>) -> NmrQueueData{
        NmrQueueData {
            conf_path,
            simulation_paths: None,
            task_queue: TaskQueue::<Task>::new(),
            conf_last_update: None,
        }
    }

    fn run(&mut self){
        loop {
            print!("waiting for simulation");
            let work = self.task_queue.get();
            let result_lock = work.get_task().result_arc;
            let simulation = work.get_task().simulation;
            let args = work.get_task().args;
            self.update_conf();
            let simulation_paths = self.simulation_paths.clone().expect("Failed to load simulation paths");
            let command = simulation_paths.get(&simulation).expect(&format!("Failed to get simulation {simulation}"));
            println!("simulating: {simulation}");

            let result = self.run_task(command, &args)
                .expect(&format!("Failed to run {simulation} with args {args:?}"));
            let rl = result_lock.lock().expect("Failed to adquire lock for writing result");
            println!("{simulation} done");
            self.task_queue.task_done(work);
        }
    }
    fn run_task(&mut self, command: &String, args: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let proc_handle = Command::new(command)
            .args(args)
            .spawn();
        match proc_handle {
            Ok(proc_handle) => {
                let output = proc_handle.wait_with_output()?;
                Ok(String::from_utf8(output.stdout)?)
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    fn was_updated(&self) -> bool {
        false
        //todo("check if configuration file was updated");
    }

    fn update_conf(&mut self) {
        if self.was_updated(){
            self.simulation_paths = Some(
                read_conf(
                    self.conf_path.as_ref().unwrap().as_str()
                )
                .expect("Error reading configuration file")
            );
        }
    }
}

pub fn read_conf(fname: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut conf = HashMap::new();
    let contents = std::fs::read_to_string(fname)?;
    for line in contents.lines() {
        let mut split = line.split(':');
        let key = split.next().unwrap();
        let value = split.next().unwrap();
        conf.insert(key.to_string(), value.to_string());
    }
    Ok(conf)
}
