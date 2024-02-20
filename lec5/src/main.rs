#![allow(unused)]

use std::marker::PhantomData;

/// Implement a function that takes in two &str slices, and returns the longest one
/// What should the signature look like?
fn longest_str() {}

/// A series of array "chunk" references
/// Cannot outlive the lifetime of the underlying array `'s`
/// You will need to store a series of array slices
#[derive(Clone, Debug)]
pub struct ArrayChunks<'s> {
    _phantom: &'s PhantomData<()>,
}

impl<'s> ArrayChunks<'s> {
    /// Returns a list of strictly-increasing chunks of the array
    pub fn increasing(arr: &'s [i32]) -> Self {
        todo!()
    }

    /// Returns a list of strictly-decreasing chunks of the array
    pub fn decreasing(arr: &'s [i32]) -> Self {
        todo!()
    }

    /// Returns a list of same-parity chunks of the array
    /// That is, chunks that are entirely even or entirely odd
    pub fn same_parity(arr: &'s [i32]) -> Self {
        todo!()
    }
}

fn main() {
    // Use this to test your `longest_str` implementation

    // let str1 = "short";
    // let str2 = "looooooong";
    // assert_eq!(longest_str(str1, str2), "looooooong");

    // Use this to test your `ArrayChunks` implementation

    let arr = [1, 2, 3, 4, 2, 3, 4, 1, 3, 7, 9, 5, 2, 6, 9];

    // The chunks should be [1, 2, 3, 4], [2, 3, 4], [1, 3, 7, 9], [5], and [2, 6, 9]
    dbg!(ArrayChunks::increasing(&arr));
}
