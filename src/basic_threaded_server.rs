#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    worker_rec: mpsc::Receiver<String>,
}

enum Message {
    NewJob(Job),
    StringJob(IoJob),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let (other_s, other_r) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let other_s = Arc::new(Mutex::new(other_s));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&other_s)));
        }

        Self {
            workers,
            sender,
            worker_rec: other_r,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

        match self.worker_rec.recv() {
            Err(_r) => {
                println!("Failed to read from worker response channel")
            }
            Ok(r) => {
                println!("Worker response: {}", r)
            }
        };
    }

    pub fn execute_s<F>(&self, f: F)
    where
        F: FnOnce(String) -> String + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::StringJob(job)).unwrap();

        match self.worker_rec.recv() {
            Err(_r) => {
                println!("Failed to read from worker response channel")
            }
            Ok(r) => {
                println!("Worker response: {}", r)
            }
        };
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

trait IoFnBox {
    fn call_box(self: Box<Self>, arg: String) -> String;
}

impl<F: FnOnce(String) -> String> IoFnBox for F {
    fn call_box(self: Box<Self>, arg: String) -> String {
        (*self)(arg)
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

type IoJob = Box<dyn IoFnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        sender: Arc<Mutex<mpsc::Sender<String>>>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job.call_box();

                    {
                        match sender.lock().unwrap().send(format!("{}: finished", id)) {
                            Err(r) => {
                                println!("Got error sending: {:#?}", r)
                            }
                            Ok(()) => {
                                println!("Sent message okay")
                            }
                        };
                    }
                },
                Message::StringJob(job) => {
                    println!("String job");
                    job.call_box("woop".into());
                    {
                        match sender.lock().unwrap().send(format!("{}: finished", id)) {
                            Err(r) => {
                                println!("Got error sending: {:#?}", r)
                            }
                            Ok(()) => {
                                println!("Sent message okay")
                            }
                        };
                    }                    
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate; executing.", id);
                    break;
                }
            }
        });

        Self {
            id: id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(200) {
        let stream = stream.unwrap();
        println!("{:#?}", stream);
        // pool.execute(|| {
        //     handle_conn(stream);
        // });
        pool.execute_s(|f| {
            println!("hello: {}", f);
            handle_conn(stream);
            "hello_world".into()
        });
    }
}


fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // something else.
    }
}
