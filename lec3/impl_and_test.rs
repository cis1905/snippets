#![allow(dead_code)]
// CIS 1905 Rust

enum TrafficLightStatus {
    Green = 0,
    Yellow,
    Red,
}

// Excercise: Write a free function that is able to take a TrafficLightStatus value
//  and then produces the next one.
fn next_light(light: TrafficLightStatus) -> TrafficLightStatus{
    todo!("please implement next_light")
}

// Exercise: Make a method on TrafficLightStatus that dynamically tells if it's time to change the light.
//  The method should a TrafficLightStatus by value and a SystemTime as inputs.
//  Then it should return a Result that is Ok with the next light if the input is
//  5 seconds old, and is Err with the old light if 5 seconds have not passed yet.
//   It should also be Err if any other errors are encountered in the block.
// The documentation for SystemTime is here
//  https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed
//   Give particular attention to the `elapsed` method, which will give you a
//   Duration that is able to tell you the number of seconds that have passed.

// Note: We unfortunately won't be able to use `?` to propagate your errors here,
//  because we are using an error type that can't be converted from some of the Results we may get.
//  Please use pattern matching constructs or existing methods on Result to break down values and define the function based on them.
//  Some pattern matching constructs include but are not limited to `match` and `if let`.
use std::time::SystemTime;
impl TrafficLightStatus {
    // Remember `this` isn't a keyword in Rust.
    // Please change this signature to be a method, and also implement the function.
    // Replace all direct mentions of TrafficLightStatus within the signature 
    fn next_light_if_available(this: TrafficLightStatus, time: SystemTime) -> Result<TrafficLightStatus, TrafficLightStatus> {
        todo!("please turn next_light_if_available into a method")
    }
}
// The power of this pattern is that we are always able to make guarantees about
//  what the computation is able to do during its running time without worrying
//  that something has gone wrong and we have two places in our code that disagree
//  on the state of the light. This would happen if we allowed the light to be
//  copied recklessly.
// There are even more powerful versions of this called the "type state" pattern
//  which let a library guarantee at compile time that a user can never take a
//  "wrong" step in a program, depending on how the library defines wrong.
//  In this current example, a user could still match on a TrafficLightStatus
//  if they really wanted or make a new one. In a type state solution, we could
//  prevent them from ever creating a Red based on a Green, or making a Yellow from nothing.


// No looking until it's implemented!
// Here we have an example of a test module.
// Unit tests in Rust often go in the same file as the original code.
// This means they have easy visibility into the things that the code was able to see.
// This attribute means to only compile this part if we're running tests.
// This is standard practice to save compile time when you're building the final product.
#[cfg(test)]
mod test_traffic_light{
    // With `use super::*;` we can now use all members of the parent module.
    use super::*;
    // the test attribute marks a test that should be run if we use `cargo test`
    #[test]
    fn test_next_light_free(){
        let statuses = [TrafficLightStatus::Green, TrafficLightStatus::Yellow, TrafficLightStatus::Red];
        let next_statuses = [TrafficLightStatus::Green, TrafficLightStatus::Yellow, TrafficLightStatus::Red];
        for (i, status) in statuses.into_iter().enumerate() {
            assert_eq!(std::mem::discriminant(&next_light(status)), std::mem::discriminant(&next_statuses[(i+1) % next_statuses.len()]))
        }
    }
    
    #[test]
    fn test_next_light_if_available(){
        use std::time::{SystemTime, Duration};
        let start = SystemTime::now();
        assert!(matches!(TrafficLightStatus::next_light_if_available(TrafficLightStatus::Green, start-Duration::from_secs(10)), Ok(TrafficLightStatus::Yellow)));
        assert!(matches!(TrafficLightStatus::next_light_if_available(TrafficLightStatus::Green, start-Duration::from_secs(1)), Err(TrafficLightStatus::Green)));
        assert!(matches!(TrafficLightStatus::next_light_if_available(TrafficLightStatus::Red, start+Duration::from_secs(10)), Err(TrafficLightStatus::Red)));
    }
}
