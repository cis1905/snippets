/**
 * OWNERSHIP PREVIEW
 *
 * Example of how iterators are lazily evaluated, so you could have an infinite iterator.
 *
 * Concepts covered:
 * - iterator trait
 * - lazy evaluation
 * - ownership: moving / borrowing / by_ref
 * - closures :D
 */
fn infinite_iterator() {
    let mut count = 0;
    // Remove `mut` keyword
    // Remove `move`
    let mut iterator = std::iter::from_fn(move || {
        count += 1;
        Some(count)
    });
 
    // Guess; what happens if you call it as is?
    // Remove `.by_ref()`. What do you think happens?
    for i in iterator.by_ref().take(10) {
        println!("{}", i);
    }

    for i in iterator.by_ref().take(10) {
        println!("{}", i);
    }

    print!("Final value of `count` variable: {}", count);
}

fn main() {
    infinite_iterator();
}
