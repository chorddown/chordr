use crate::configuration::Configuration;
use crate::error::Error;
use crate::task::{RecurringTaskTrait, TaskTrait};

pub struct CollectionTask<'a> {
    tasks: Vec<&'a dyn RecurringTaskTrait>,
}

impl<'a> CollectionTask<'a> {
    pub fn new(tasks: Vec<&'a dyn RecurringTaskTrait>) -> Self {
        Self { tasks }
    }
}

impl<'a> TaskTrait for CollectionTask<'a> {
    fn with_configuration(_configuration: Configuration) -> Result<Self, Error>
    where
        Self: std::marker::Sized,
    {
        unimplemented!()
    }
}

impl<'a> RecurringTaskTrait for CollectionTask<'a> {
    fn run(&self) -> Result<(), Error> {
        for task in &self.tasks {
            if let Err(e) = task.run() {
                return Err(e);
            }
        }
        Ok(())
    }
}
