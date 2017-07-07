// http://rosettacode.org/wiki/Handle_a_signal
// 
// Provide a program that displays an integer on each line of output at the rate of
// about one per half second. Upon receipt of the SIGINT signal 
// (often generated by the user typing ctrl-C ( or better yet, SIGQUIT ctrl-\ )) 
// the program will cease outputting integers, output the number of seconds the 
// program has run, and then the program will quit. 


#[macro_use]
extern crate chan;
extern crate chan_signal;

use std::thread;
use std::time::{Duration, Instant};

use chan_signal::Signal;

fn main() {
    // Signal gets a value when the OS sent a INT or TERM signal.
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    // When our work is complete, send a sentinel value on `sdone`.
    let (sdone, rdone) = chan::sync(0);
    // Run work.
    let now = Instant::now();

    thread::spawn(move || run(sdone));

    // Wait for a signal or for work to be done.
    chan_select! {
        signal.recv() -> signal => {
            println!("\nreceived signal: {:?}", signal.unwrap());
            println!("Execution took {} secs", now.elapsed().as_secs());
        },
        rdone.recv() => {}
    }
}

fn run(_sender: chan::Sender<()>) {
    let mut n = 0;
    loop {
        println!("{}", n);
        n += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
