use std::collections::BTreeMap;
use std::fmt;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticket(usize);

pub struct Signal<'a, A, B> {
    index: AtomicUsize,
    slots: Mutex<BTreeMap<Ticket, Box<FnMut(&A) -> B + 'a>>>
}

impl<'a, A, B> Signal<'a, A, B> {
    pub fn new() -> Signal<'a, A, B> {
        Signal {
            index: AtomicUsize::new(0),
            slots: Mutex::new(BTreeMap::new())
        }
    }

    pub fn connect<F: FnMut(&A) -> B + 'a>(&self, f: F) -> Ticket {
        let     index = self.index.fetch_add(1, Ordering::SeqCst);
        let mut slots = self.slots.lock().unwrap();
        slots.insert(Ticket(index), Box::new(f));
        Ticket(index)
    }

    pub fn disconnect(&self, t: Ticket) {
        self.slots.lock().unwrap().remove(&t);
    }

    pub fn emit(&self, a: A) -> Option<B> {
        let mut x = None;
        let mut slots = self.slots.lock().unwrap();
        for f in slots.values_mut() {
            x = Some(f(&a))
        }
        x
    }
}

impl<'a, A, B> fmt::Debug for Signal<'a, A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signal {{ index: {:?} }}", self.index)
    }
}

