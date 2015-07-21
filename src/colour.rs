#![allow(dead_code, unused_must_use)]
use std::sync::mpsc::{channel, Sender, Receiver};
use super::well;

struct Actor<M>{mailbox: Receiver<M>}

impl<M> Actor<M> {
    fn receive<Message,F>(&mut self, f:F) where F: FnOnce(M){
        match self.mailbox.recv() {
            Ok(message) => f(message),
            Err(err) => println!("Received channel error: {}", err),
        }
    }
}

fn spawn<F>(f:F) -> Sender<i32> where F: FnOnce(Actor<i32>) + Send + 'static{
    let(address, mbox) = channel::<i32>();
    well::spawn(move || {
        let actor = Actor{mailbox: mbox};
        f(actor);
    });
    address
}

macro_rules! actor {
    ($($fun:ident($($args:expr),*));*;) => {
        |mut actor: Actor<_>| {
            $(actor.$fun::<i32,_>($($args),*);)*;
        }
    }
}

fn main() {
    let actor = actor! {
        receive(|m| {m + 2;});
        receive(|m| {m + 3;});
    };
    let address = spawn(actor);
    address.send(1);
}
