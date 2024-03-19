#![allow(unused)]

use std::collections::VecDeque;
use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::Add;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use tokio::join;

////////////////////////
// MUTEXES (MUTICES?) //
////////////////////////

/*

1. MULTI-THREADED COUNTER

*/

// Let's start by implementing a simple multi-threaded counter

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
    }

    // Now, we need to join all of the threads we made to the main thread to
    // make sure the final value is available...
    for handle in join_handles {
        // TODO: join the handle to the main thread
    }

    return *counter.lock().unwrap();
}

/*

2. PRDUCER/CONSUMER TASK QUEUE

*/

// Let's get a little bit fancier... Another common use case for mutexes is
// a producer/consumer queue. As the name implies, the procer produces values
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
            // 1. Now that there's a line to read, acquire a lock on the queue
            // 2. Parse the line as an integer
            // 3. Clear the line buffer
            // 4. Check to make sure the integer parsed correctly
            // 5. Add the integer to the queue
            // 6. Then call cvar.notify_one(); to notify one consumer that there
            //    is a value waiting to be consumed
        }
    });

    let for_consumer = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*for_consumer;
        loop {
            // 1. Acquire a lock on the queue
            // 2. While the queue is empty, do the following:
            //
            //    queue = cvar.wait(queue).unwrap();
            //
            //    cvar.wait(queue) will release the lock on the queue and halt
            //    the thread until it is notified. At that point, it'll
            //    re-acquire the lock and return the locked queue!
            //
            // 3. Then, do the computation (* 10) and print the result
            // 4. BONUS: if you do things right, you should be able to insert
            //    a delay into your computation and still queue values from
            //    the producer. To insert a 1-sec delay, use:
            //
            //    thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    // EVEN MORE BONUS: you can add multiple consumers! Try it out. Make a
    // function which creates a new consumer and returns its join handle. Give
    // each consumer a unique ID and print it out with the output.

    // Here, I'll join the threads for you :)
    producer.join();
    consumer.join();
}

/*

3. ASYNC/AWAIT

*/

// If you are working with a highly concurrent project or need more flexibility
// async/await syntax can be very convenient. Remember that an async function is
// really just a state machine that returns a Future. Each time you call .await,
// the state machine is paused and the Future is returned to the executor to
// handle.

/// This function sleeps for one second then returns the sum of `x` and `y`
/// The `Send` and `Sync` traits make sure `T` and `U` can be sent across
/// threads safely
async fn expensive_add<T, U>(x: T, y: U) -> T::Output
where
    T: Send + Sync + Add<U>,
    U: Send + Sync,
{
    tokio::time::sleep(Duration::from_secs(1)).await;
    x + y
}

/// This function takes a slice of tuples and adds each pair of elements,
/// printing the result of each operation
async fn add_many<T, U>(inputs: &[(T, U)]) -> &'static str
where
    T: Send + Sync + Display + Clone + Add<U>,
    U: Send + Sync + Display + Clone,
    T::Output: Display,
{
    // TODO: Use a for loop and async/await syntax to call expensive_add on each
    // input pair of T's, then print the result of each addition

    "done!"
}

async fn add_parallel() {
    // If you just use async/await, the code will execute sequentially (i.e.
    // one line after another). Recall that the return type of an async function
    // is actually a Future. For example, the return type of expensive_add is
    // Future<Output = T>, and for add_many it is Future<Output = ()>. The tokio
    // crate provides a convenient way to "combine" futures into a single future
    // that runs concurrently. This way, we can call add_many several times and
    // get the results in parallel.

    // Here we define the inputs to call our function with
    let ints = [(1, 2), (3, 4), (5, 6)];
    let floats = [(1.4, 2.5), (3.6, 4.7), (5.8, 6.9)];
    let strings = [
        ("hello".to_owned(), "world"),
        ("foo".to_owned(), "bar"),
        ("baz".to_owned(), "qux"),
    ];

    // The `join!` macro takes a list of futures and returns a tuple of their
    // outputs there is an implicit `await` when you use `join!`
    let (msg1, msg2, msg3) = join!(add_many(&ints), add_many(&floats), add_many(&strings));
}

#[tokio::main]
async fn main() {
    // Uncomment after task 1, should print "Counter value is: 10"
    // println!("Counter value is: {}", shared_counter(10));

    // Uncomment after task 2, should be able to write in values and see the
    // result of the computation printed
    // launch_multiplier();

    // Uncomment after task 3, should print the result of add_many in parallel
    // add_parallel().await;
}
