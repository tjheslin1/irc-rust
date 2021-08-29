use std::fmt;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
pub struct Worker {
    id: usize,
    pub handle: Option<thread::JoinHandle<()>>,
}

#[derive(Debug)]
pub struct ThreadPool {
    pub workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Debug, PartialEq)]
pub struct PoolCreationError {
    pub message: String,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PoolCreationError: {}", self.message)
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Error obtaining lock.")
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(thread),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError {
                message: "Error creating ThreadPool, cannot create a pool with zero threads!"
                    .to_string(),
            }),
            n => {
                let (sender, r) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(r));

                let workers: Vec<Worker> = (0..n)
                    .into_iter()
                    .map(|i| Worker::new(i, Arc::clone(&receiver)))
                    .collect();

                Ok(ThreadPool { workers, sender })
            }
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap()
            }
        }
    }
}
