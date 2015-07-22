#![allow(unused_must_use)]
#[macro_use]
extern crate libcolour;

use libcolour::colour::*;

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
    spawn(recurse_actor());
    let send = spawn(recurse_actor());
    for i in 1..10 {
        let sender = send.clone();
        ::std::thread::spawn(move || {
            for j in (1+ 1000*(i - 1))..(1000 + 1000*(i-1)) {
                sender.send(j);
            }
        });
    }
    ::std::thread::sleep_ms(2000);
    send.send(0);
}
