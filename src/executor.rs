use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use anyhow::{anyhow, Result};

trait Task: Send + 'static {
    fn get_name(&self) -> String;
    fn task(&self) -> impl Fn() -> () + Send + 'static;
}

pub struct Executor {
    jobs: HashMap<String, JoinHandle<()>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn spawn<F, A>(&mut self, task_name: String, task: F, args: A) -> Result<()>
    where
        F: FnOnce(A) -> (),
        F: Send + 'static,
        A: Send + 'static,
    {
        let handler = thread::spawn(move || task(args));
        self.jobs.insert(task_name, handler);
        Ok(())
    }

    pub fn join(&mut self) -> Result<()> {
        for (task_name, task_handler) in self.jobs.drain().into_iter() {
            let join_result = task_handler.join();
            if join_result.is_err() {
                return Err(anyhow!("Error while finishing task `{}`", task_name));
            }
        }
        Ok(())
    }

    pub fn execute(&mut self, task: impl Task) -> Result<()> {
        let task_name = (&task.get_name()).clone();
        let handler = thread::Builder::new()
            .name(task.get_name())
            .spawn(task.task())?;
        self.jobs.insert(task_name, handler);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    use super::{Executor, Task};

    struct TestTask;
    impl Task for TestTask {
        fn get_name(&self) -> String {
            String::from("test_task")
        }

        fn task(&self) -> impl Fn() -> () + Send + 'static {
            move || {
                println!("Hello from inside");
            }
        }
    }

    #[test]
    fn test_1() {
        let test_obj = TestTask;

        let mut executor = Executor::new();

        executor.execute(test_obj);

        thread::sleep(Duration::from_secs(1));
        executor.join();

        assert!(1 == 1);
    }
}
