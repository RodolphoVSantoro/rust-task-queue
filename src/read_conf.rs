use std::collections::HashMap;
use std::error::Error;

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