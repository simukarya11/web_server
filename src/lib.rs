use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::Message::Terminate;


// ************************* ThreadPool *************************
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending termination command for all threads");

        for _ in &mut self.workers {
            self.sender.send(Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker for thread: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f:F) where F: FnOnce() + Send + 'static{
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// ************************* Worker *************************

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message =  receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a message; executing", id);
                match message {
                    Message::NewJob(job) => {job();}
                    Message::Terminate => {
                        println!("Worker {} was told to terminate;", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}

// ************************* Job *************************
type Job = Box<dyn FnOnce() + Send + 'static>;


// ************************* Message *************************
enum Message {
    NewJob(Job),
    Terminate
}
