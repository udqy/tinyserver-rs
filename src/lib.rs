use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
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
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(e) = self.sender.as_ref().unwrap().send(job) {
            eprintln!("Failed to send job to worker: {}", e);
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let guard = match receiver.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    eprintln!("Worker {id} failed to acquire lock: {}", e);
                    break;
                }
            };

            // Attempt to receive a job
            match guard.recv() {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    // Catch any panics during job execution
                    if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| job())) {
                        eprintln!("Worker {id} encountered a panic: {:?}", e);
                    }
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    eprintln!("Worker {} failed to shut down cleanly: {:?}", worker.id, e);
                }
            }
        }
    }
}
