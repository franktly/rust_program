use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// public
/// Create a `ThreadPool` struct to manage all worker thread including creating and send message
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// Create a `Job` type alias for a Box that holds each closure and then sending the job down the
type Job = Box<dyn FnOnce() + Send + 'static>;

// Signaling to the Threads to Stop Listening for Jobs
enum Message {
    /// NewJob: takes a `Job` type value to notify worker thread to process a new job
    NewJob(Job),
    /// Terminate: notify worker thread to exit
    Terminate,
}

// private
struct Worker {
    /// id: worker idendification
    /// thread: worker thread handle with `Option` type to moved out by `drop` trait implemented by
    /// `ThreadPool`
    id: usize,
    // thread: thread::JoinHandle<()>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new Worker
    ///  
    ///  id: the worker idendification
    ///  receiver: the `mpsc` receive port with the Arc<Mutex>> type to avoid data race
    ///
    ///  The `new` function will receive the `Message` from `ThreadPool::execute` function and
    ///  behave according to the message type, if `NewJob` type it will spawn a thread wait for in
    ///  a loop
    ///  new job to process , if `Terminate` it will break and exit current created thread

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock to acquire the mutex and unwrap to panic on any errors
            // recv to receive a Job from the channel, recv call will block until a job becomes
            // available on the current thread
            // The Mutex<T> ensures that only one Woker thread at a time is trying to request a job
            //
            // v1: receiver;
            // v2: let job = receiver.lock().unwrap().recv().unwrap();
            // v2: println!("Worker {} got a job; executing.", id);
            // v2: job();

            let message = receiver.lock().unwrap().recv().unwrap();

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
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///  
    ///  The size is the number of threads in the pool
    ///
    ///  # Panics
    ///
    ///  The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // clone the `Arc` to bump the reference count so the workers can share ownership of
            // the receiving end
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// execute function
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// implementing the Drop Trait on ThreadPool
/// Implement the `drop` trait on ThreadPool
/// The `drop` function will first end the thread's number's `Message::Terminate` signal and then
/// use `thread::join` to wait all worker's thread to exit
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // move the thread out of worker by Option type not raw thread::JoinHandle type
            // v1: worker.thread.join().unwrap();
            // take method on Option takes the Some variant out and leaves None in its place
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
