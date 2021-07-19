use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

pub struct DispatcherTimer {
    is_running: Arc<AtomicBool>,
    sender: Sender,
}

impl DispatcherTimer {
    pub fn new(interval_ms: u64, tick: impl Fn() -> bool + Send + 'static) -> Self {
        let (sender, receiver) = rendevous_channel();
        let is_running = Arc::new(AtomicBool::new(false));
        let timer_is_running = Arc::clone(&is_running);

        thread::spawn(move || {
            while receiver.receive().is_ok() {
                loop {
                    thread::sleep(Duration::from_millis(interval_ms));
                    if timer_is_running.load(Ordering::Acquire) && tick() {
                        timer_is_running.store(false, Ordering::Release);
                        break;
                    }
                }
            }
        });

        Self { is_running, sender }
    }

    pub fn is_enabled(&self) -> bool {
        self.is_running.load(Ordering::Acquire)
    }

    pub fn start(&self) -> bool {
        match self
            .is_running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire)
        {
            Ok(_) => self.sender.send().is_ok(),
            Err(_) => false,
        }
    }

    pub fn stop(&self) -> bool {
        self.is_running
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::Acquire)
            .is_ok()
    }
}

/// Instead of passing data between threads, sender and receiver is used to wake up a thread from the parent thread.
fn rendevous_channel() -> (Sender, Receiver) {
    let shared = Shared {
        queue: Mutex::new(()),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: Arc::clone(&shared),
        },
        Receiver { shared },
    )
}

struct Shared {
    available: Condvar,
    queue: Mutex<()>,
}

struct Sender {
    shared: Arc<Shared>,
}

struct Receiver {
    shared: Arc<Shared>,
}

impl Sender {
    pub fn send(&self) -> Result<(), ()> {
        match self.shared.queue.try_lock() {
            Ok(lock) => {
                drop(lock);
                self.shared.available.notify_one();
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl Receiver {
    pub fn receive(&self) -> Result<(), ()> {
        loop {
            return match self.shared.queue.lock() {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            };
        }
    }
}
