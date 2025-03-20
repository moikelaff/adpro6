use std::sync::{mpsc, Arc, Mutex};

use crate::worker::Worker;

// Make Message enum public so worker.rs can access it
pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender) 
        }
    }

    /// Create a ThreadPool with a builder pattern approach.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = ThreadPool::build()
    ///     .with_size(4)
    ///     .finalize();
    /// ```
    pub fn build() -> ThreadPoolBuilder {
        ThreadPoolBuilder::new()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        
        // Send terminate message to all workers
        for _ in &self.workers {
            if let Some(sender) = &self.sender {
                sender.send(Message::Terminate).unwrap();
            }
        }
        
        println!("Shutting down all workers.");
        
        // Wait for all workers to finish their tasks and terminate
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct ThreadPoolBuilder {
    size: Option<usize>,
}

impl ThreadPoolBuilder {
    fn new() -> Self {
        ThreadPoolBuilder { size: None }
    }

    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn finalize(self) -> ThreadPool {
        let size = self.size.unwrap_or(num_cpus::get());
        ThreadPool::new(size)
    }
}