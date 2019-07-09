use std::thread;
use std::sync::mpsc;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        ThreadPool
    }
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
        workers.push(Worker::new(id));
    }

    ThreadPool{
        workers
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver:mpsc::Receiver<Job>) -> Worker{
        let thread = thread::spawn( || {
            receiver;
        });
        Worker{
            id,
            thread,
        }
    }
}

