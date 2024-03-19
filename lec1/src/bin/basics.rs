#![allow(dead_code)]

use std::{thread, time::Duration};

fn main() {
    // ---------------------------- Basic Syntax -------------------------------
    let x = 0;
    let mut y = 5;

    // Primitives: integers (e.g. usize, u32, i32), floats, bool, char

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Arrays: fixed length
    let a = [1, 2, 3, 4, 5];

    // Vectors: variable length (Python list, C++ std::vector, Java ArrayList)
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    let v2 = vec![1, 2, 3];
}

/// ---------------------------- Functions -------------------------------
/// Return keyword is optional if you omit ;. See example:
fn five(input: i32) -> i32 {
    5 + input
}

// Make sure you omit ; at the end if you are using the simplified return!
fn control_flow() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    if 1 == 2 {
        println!("whoops, mathematics broke");
    } else {
        println!("everything's fine!");
    }

    let string = "hello there!";
    for (index, c) in string.chars().enumerate() {
        println!("{}: {}", index, c);
    }
}

// In Rust, you can pattern match anything with a `match` statement:
fn is_palindrome(items: &[char]) -> bool {
    match items {
        [first, middle @ .., last] => first == last && is_palindrome(middle),
        [] | [_] => true,
    }
}

// Rust Anonymous Functions that capture their environment in 3 ways:
// 1) borrowing immutably, 2) borrowing mutably, 3) taking ownership
fn closure() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(5);
}
