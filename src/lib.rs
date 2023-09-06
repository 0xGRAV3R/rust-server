use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool. 
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create threads
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>

}


impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

    }
}
