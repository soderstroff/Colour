#![allow(dead_code, unused_must_use)]

use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

fn read_from_mailbox(mbox: Receiver<i64>) {
    loop {
        mbox.recv();
    }
}

fn send_to_mailbox(mbox: Sender<i64>, i:i64) {
    loop {
        mbox.send(i);
    }
}
macro_rules! spawn {
    ($($case:pat => $result:expr),*) => {
        {
            let(address, mbox) = channel();
            thread::spawn(move || {
                loop {
                    match mbox.recv() {
                        Ok(message) => {
                            match message {
                                $($case => $result),*
                            }
                        }
                        Err(_) => println!("No more senders"),
                    }
                }
            });
            address
        }
    };
}
fn main() {
    let address: Sender<i32> = spawn!{
        1 => (),
        _ => break
    };

    address.send(1);
}

/*Are actors monads, or comonads?
monad m supports bind :: m a -> (a -> m b) -> m b and return :: a -> m a
comonad w supports extend :: a -> (w a -> b) -> w b and extract :: w a -> a

Monad
divide2byX x = if x != 0 then Some(2 / x) else Nothing
24 >>= divide2byX >>= \x case x of Some(a) divide2byX a else Nothing

Comonad
blur k (P i a)  = .5*k*a!(i-1) + k*a!i + .5*k*a!(i+1)
P 0 x ==> fmap (+1) =>> blur 24 =>> blur .5 ==> fmap(* 4)

In the first, plumbing between sequential computation was abstracted, binding results to functions.
In the second, computation over an entire context was abstracted, extending reductions over contexts.

data Mailbox<M>;*/
