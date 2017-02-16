use std::mem;
use std::sync::Arc;
use std::sync::mpsc::Receiver;

use futures::{Async, Future};
use futures::executor::{self, Spawn, Unpark};
use gtk::{self, Continue};

pub struct Loop {
    unpark:   Arc<UnparkNone>,
    tasks_a:  Vec<Spawn<Box<Future<Item=(), Error=()>>>>,
    tasks_b:  Vec<Spawn<Box<Future<Item=(), Error=()>>>>,
    receiver: Receiver<Box<Future<Item=(), Error=()>>>
}

impl Loop {
    pub fn new(rx: Receiver<Box<Future<Item=(), Error=()>>>) -> Loop {
        Loop {
            unpark:   Arc::new(UnparkNone()),
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
                if let Ok(Async::NotReady) = s.poll_future(self.unpark.clone()) {
                    self.tasks_b.push(s)
                }
            }
            mem::swap(&mut self.tasks_a, &mut self.tasks_b);
            Continue(true)
        });
    }
}

struct UnparkNone();

impl Unpark for UnparkNone {
    fn unpark(&self) {}
}

