extern crate toml;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use toml::Value;

#[derive(Debug)]
pub struct Config {
    config: String,
    env_file: String,
}

impl Config {
    pub fn new(config: &str, env_file: &str) -> Config {
        Config {
            config: String::from(config),
            env_file: String::from(env_file),
        }
    }

    pub fn config(&self) -> &String {
        &self.config
    }

    pub fn env_file(&self) -> &String {
        &self.env_file
    }
}

pub fn run(conf: Config) {
    let mut file = File::open(conf.config()).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let value = contents.parse::<Value>().unwrap();

    let mut envs = String::new();
    iter_table(&value, &mut envs);
    let mut env_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&conf.env_file)
        .unwrap();
    if let Err(_) = env_file.write(envs.as_bytes()) {
        eprintln!("Write content to {} failed!", &conf.env_file);
    }
}

fn iter_table(value: &Value, envs: &mut String) {
    if let Some(table) = value.as_table() {
        for (k, v) in table {
            if v.is_table() {
                iter_table(v, envs);
            } else {
                if v.is_str() {
                    envs.push_str(&format!(
                        "{}={}\n",
                        k,
                        v.as_str().unwrap().trim_matches('"')
                    ));
                } else {
                    envs.push_str(&format!("{}={}\n", k, v));
                }
            }
        }
    }
}
