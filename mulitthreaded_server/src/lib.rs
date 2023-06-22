use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{JoinHandle, self}};


struct Worker {
    id:usize,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap_or_else(|err| 
                panic!("{err}")).recv().unwrap_or_else(|err| panic!("{err}"));
            job();
            
        });
        Worker { id: id, thread:Some(thread) }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce()+Send+'static >;

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
        let (tx,rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers: Vec<Worker> = Vec::<Worker>::with_capacity(size);
        for val in 0..size {
            workers.push(Worker::new(val,Arc::clone(&rx.clone()) ))
        }
        ThreadPool { workers: workers, sender: Some(tx) }
    }

    pub fn execute<F>(&self, f:F) 
        where
            F: FnOnce() + Send + 'static,{
                let job = Box::new(f);
                self.sender.as_ref().unwrap().send(job).unwrap();
    }
} 

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         drop(self.sender.take());
//         for worker in &mut self.workers {
//             println!("Shutting down worker {}", &worker.id);
//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }