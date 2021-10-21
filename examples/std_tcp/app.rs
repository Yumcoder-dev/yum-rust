use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // reading_the_request();
    building_thread_pool_server();
}

////////////////////////////////////////////////////////////////
// telnet localhost 7878
//
// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
// Firefox/52.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate
// Connection: keep-alive
// Upgrade-Insecure-Requests: 1
//
fn reading_the_request() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//
//     let get = b"GET / HTTP/1.1\r\n";
//
//     if buffer.starts_with(get) {
//         let contents = fs::read_to_string("hello.html").unwrap();
//
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//             contents.len(),
//             contents
//         );
//
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     } else {
//         // some other request
//     }
// }

////////////////////////////////////////////////////////////////
fn building_a_single_threaded_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

////////////////////////////////////////////////////////////////
fn building_multi_thread_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Spawning a new thread for each stream
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

////////////////////////////////////////////////////////////////
fn building_thread_pool_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// The F type parameter also has the trait bound Send and the lifetime bound 'static,
// which are useful in our situation: we need Send to transfer the closure from one
// thread to another and 'static because we don’t know how long the thread will take to execute.
type Job = Box<dyn FnOnce() + Send + 'static>; // FnOnce = function or closure

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

        ThreadPool { workers, sender }
    }

    // The F type parameter also has the trait bound Send and the lifetime bound 'static,
    // which are useful in our situation: we need Send to transfer the closure from one
    // thread to another and 'static because we don’t know how long the thread will take to execute.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job();
            }
        });

        Worker { id, thread }
    }
}
