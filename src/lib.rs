use std::thread;

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            handle: thread::spawn(|| {}),
        } 
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id))
        }

        Self { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
    }
}
