#![allow(dead_code, unused_must_use)]
use std::sync::mpsc::{channel, Sender, Receiver};
use super::well;

struct Actor<M:Send> { mbox: Receiver<M> }

impl <M:Send> Actor<M> {
    pub fn recv(&mut self) -> M {
        match self.mbox.recv() {
            Ok(message) => message,
            Err(err) => panic!("Encountered error: {}", err),
        }
    }
}

struct Behavior<M:Send>(Option<Box<Fn(&mut Actor<M>) -> Behavior<M>>>);

#[inline]
fn end<M:Send>() -> Behavior<M> {
    Behavior(None)
}

macro_rules! behavior {
    ($closure:expr) => {
        Behavior(Some(Box::new(
            $closure
        )));
    }
}

unsafe impl <M> Send for Behavior<M> { }

fn spawn<M:'static + Send>(b: Behavior<M>) -> Sender<M> {
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

fn recurse_actor() -> Behavior<i32> {
    behavior!(
        |actor| {
            match actor.recv() {
                0 => {println!("Buh-bye!"); end()},
                i@_ => {println!("Got a {}!", i); recurse_actor()},
            }
        }
)}

fn main() {

    let send = spawn(recurse_actor());
    for i in 1..10 {
        let sender = send.clone();
        well::spawn(move || {
            for j in (1+ 1000*(i - 1))..(1000 + 1000*(i-1)) {
                sender.send(j);
            }
        });
    }
    ::std::thread::sleep_ms(2000);
    send.send(0);
}
