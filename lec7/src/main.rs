#![allow(unused)]

use std::collections::VecDeque;
use std::num::ParseIntError;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

////////////////////////
// MUTEXES (MUTICES?) //
////////////////////////

/*

1. MULTI-THREADED COUNTER

*/

// Let's start by implementing a simple multi-threaded counter

/// Counts up to n using n threads
fn shared_counter(n: usize) -> i32 {

    // We'll start by declaring a counter using an atomically reference counted
    // mutex, initially containing the integer zero
    let counter = Arc::new(Mutex::new(0));

    // We'll then make a vector of JoinHandles so we can wait for all of our
    // threads
    let mut join_handles: Vec<JoinHandle<()>> = Vec::with_capacity(n);

    for _ in 0..n {
        // TODO: make a new clone of the counter and send it to a new thread.
        // Within that thread, increment the counter by 1. Don't forget to
        // push the thread's handle onto the join_handles vec!
        let counter_clone = Arc::clone(&counter);
        let t = thread::spawn(move || {
            let mut guard = counter_clone.lock().unwrap();
            *guard += 1;
        });
        join_handles.push(t);
    }

    // Now, we need to join all of the threads we made to the main thread to
    // make sure the final value is available...
    for handle in join_handles {
        // TODO: join the handle to the main thread
        handle.join().unwrap();
    }

    return *counter.lock().unwrap();
}

/*

2. PRDUCER/CONSUMER TASK QUEUE

*/

// Let's get a little bit fancier... Another common use case for mutexes is
// a producer/consumer pair. As the name implies, the procer produces values
// and the consumer consumes them. For example, let's imagine we want a program
// to listen to input from the user and multiply that input by 10. In reality,
// the computation may be really expensive, and we don't want to hang on the
// thread that's listening for input while the computation is happening!

fn launch_multiplier() {

    // We'll start by declaring our queue, which is a deque and a condition
    // variable...
    // What's a condition variable? It allows us to signal accross threads that
    // new values are available in the deque without requiring the consumer to
    // acquire a lock on the deque directly. 
    let queue: Arc<(Mutex<VecDeque<i32>>, Condvar)> = 
        Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

    let for_producer = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*for_producer;
        let stdin = std::io::stdin();
        let mut line_buf = String::new();
        while let Ok(_) = stdin.read_line(&mut line_buf) {
            let mut queue = lock.lock().unwrap();
            let int: Result<i32, _> = line_buf.trim().parse();
            line_buf.clear();

            if let Err(_) = int {
                println!("That's not an int! Try again.");
                continue;
            }

            queue.push_back(int.unwrap());
            cvar.notify_one();
        }
    });

    let for_consumer = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*for_consumer;
        loop {
            let mut queue = lock.lock().unwrap();
            while queue.is_empty() {
                queue = cvar.wait(queue).unwrap();
            }
            let value = queue.pop_front().unwrap();
            println!("Consumed {}, result is {}", value, value * 10);
        }
    });

    producer.join();
    consumer.join();
}

fn main() {
    println!("Counter value is: {}", shared_counter(10));

    launch_multiplier();
}
