use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
// 任务函数
type Job = Box<dyn FnOnce() + Send + 'static>;
// 线程池消息
enum Message {
    NewJob(Job),
    Terminate,
}
// 工作者
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
// 线程池本体
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move|| {
            loop {
                let message = receiver.lock().unwrap().recv().expect("消息接受失败");
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }
            }
        });
        Worker {
            id,
            thread:Some(thread)
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, String> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let mut workers = Vec::with_capacity(size);
            let receiver = Arc::new(Mutex::new(receiver));
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool { workers, sender})
        } else {
            Err("线程数必须大于0".to_string())
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).expect("线程池发送失败");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}