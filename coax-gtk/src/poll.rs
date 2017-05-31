use std::mem;
use std::sync::mpsc::Receiver;

use futures::{Async, Future};
use futures::executor::{self, Spawn, Notify};
use gtk::{self, Continue};

const NOTIFY_NONE: &NotifyNone = &NotifyNone();

pub struct Loop {
    unpark:   &'static NotifyNone,
    tasks_a:  Vec<Spawn<Box<Future<Item=(), Error=()>>>>,
    tasks_b:  Vec<Spawn<Box<Future<Item=(), Error=()>>>>,
    receiver: Receiver<Box<Future<Item=(), Error=()>>>
}

impl Loop {
    pub fn new(rx: Receiver<Box<Future<Item=(), Error=()>>>) -> Loop {
        Loop {
            unpark:   NOTIFY_NONE,
            tasks_a:  Vec::new(),
            tasks_b:  Vec::new(),
            receiver: rx
        }
    }

    pub fn start(mut self) {
        gtk::timeout_add(100, move || {
            for f in self.receiver.try_iter() {
                self.tasks_a.push(executor::spawn(f))
            }
            for mut s in self.tasks_a.drain(..) {
                if let Ok(Async::NotReady) = s.poll_future_notify(&self.unpark, 0) {
                    self.tasks_b.push(s)
                }
            }
            mem::swap(&mut self.tasks_a, &mut self.tasks_b);
            Continue(true)
        });
    }
}

struct NotifyNone();

impl Notify for NotifyNone {
    fn notify(&self, _: usize) {}
}

