#![allow(dead_code, unused_must_use)]
use std::sync::mpsc::{channel, Sender, Receiver};
use super::well;

struct Actor<M:Send> { mbox: Receiver<M> }



pub struct Behavior<M:Send>(pub Option<Box<Fn(&mut Actor<M>) -> Behavior<M>>>);

impl <M:Send> Actor<M> {
    pub fn recv(&mut self) -> M {
        match self.mbox.recv() {
            Ok(message) => message,
            Err(err) => panic!("Encountered error: {}", err),
        }
    }
}
#[inline]
pub fn end<M:Send>() -> Behavior<M> {
    Behavior(None)
}

#[macro_export]
macro_rules! behavior {
    ($closure:expr) => {
        Behavior(Some(Box::new(
            $closure
        )));
    }
}

unsafe impl <M> Send for Behavior<M> { }

pub fn spawn<M:'static + Send>(b: Behavior<M>) -> Sender<M> {
    let (address, mbox) = channel::<M>();
    let mut actor = Actor { mbox: mbox };

    well::spawn(move || {
        if let Some(p_to_c) = b.0 {
            let mut c = p_to_c;
            while let Some(next) = ((*c)(&mut actor)).0 {
                c = next;
            }
        }
    });

    address
}
