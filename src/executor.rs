use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use anyhow::{anyhow, Result};

struct Executor {
    jobs: HashMap<String, JoinHandle<()>>,
}

impl Executor {
    fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    fn spawn(&mut self, task_name: String, task: fn() -> ()) -> Result<()> {
        let handler = thread::spawn(move || task());
        self.jobs.insert(task_name, handler);
        Ok(())
    }

    fn join(&mut self) -> Result<()> {
        for (task_name, task_handler) in self.jobs.drain().into_iter() {
            let join_result = task_handler.join();
            if join_result.is_err() {
                return Err(anyhow!("Error in finishing task {}", task_name));
            }
        }
        Ok(())
    }
}
