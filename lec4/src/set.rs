#![allow(unused)]

use std::{marker::PhantomData, slice::Iter};

// We define

/// A generic trait for a mathematical set containing elements of type `T`
/// May or may not have a finite number of elements
///
/// For example, a set may look like {A, B, C} or {"hello", "world"} or {0, 1, 2, ...}
pub trait Set<T> {
    /// Returns `true` iff the set contains the given element, and `false` otherwise
    fn contains(&self, element: &T) -> bool;
}

/// The empty set, defined to contain no elements
pub struct Empty;
impl<T> Set<T> for Empty {
    // Since the set is empty, we always return `false`
    fn contains(&self, _: &T) -> bool {
        false
    }
}

/// The singleton set, defined to contain exactly one element
///
/// Defined to be S = {e}, where e is the singleton element
pub struct Singleton<T>(pub T);
impl<T: Eq> Set<T> for Singleton<T> {
    // We return `true` iff the passed in element equals the singleton element
    fn contains(&self, element: &T) -> bool {
        *element == self.0
    }
}

/// The universal set, defined to contain all elements
///
/// Depending on the type `T`, may be of infinite cardinality
pub struct Universal;
impl<T> Set<T> for Universal {
    // Since the set is universal, we always return `true`
    fn contains(&self, _: &T) -> bool {
        true
    }
}

/// A set with a finite number of elements, stored internally with a `Vec`
/// The order of the elements should not matter
pub struct VecSet<T> {
    /// The elements contained within the set
    elements: Vec<T>,
}
impl<T> VecSet<T> {
    /// Constructs a new `VecSet` with the given elements
    pub fn from_vec(elements: Vec<T>) -> Self {
        todo!()
    }
}
impl<T: Clone> VecSet<T> {
    /// Constructs a new `VecSet` with the given elements
    pub fn from_slice(elements: &[T]) -> Self {
        todo!()
    }
}
impl<T> Set<T> for VecSet<T> {
    fn contains(&self, element: &T) -> bool {
        todo!()
    }
}
impl<T> PartialEq for VecSet<T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
impl<T> Eq for VecSet<T> {}

// TODO: implement basic set operations

// Before this, we will need a quick brief on `PhantomData` (snippet taken from docs)
//
// Zero-sized type used to mark things that "act like" they own a `T`.
//
// Adding a `PhantomData<T>` field to your type tells the compiler that your
// type acts as though it stores a value of type `T`, even though it doesn't
// really. This information is used when computing certain safety properties.
//
// Though they both have scary names, `PhantomData` and 'phantom types' are
// related, but not identical. A phantom type parameter is simply a type
// parameter which is never used. In Rust, this often causes the compiler to
// complain, and the solution is to add a "dummy" use by way of `PhantomData`.

// UNION

/// The union of two sets (i.e. the set containing elements in s1 or s2)
///
/// For example:
/// Let A = {0, 1}
/// Let B = {1, 2}
///
/// Union(A, B) = {0, 1, 2}
pub struct Union<T, S1: Set<T>, S2: Set<T>> {
    s1: S1,
    s2: S2,
    phantom: PhantomData<T>, // See what happens if you remove this
}
impl<T, S1: Set<T>, S2: Set<T>> Union<T, S1, S2> {
    fn new(s1: S1, s2: S2) -> Self {
        Self {
            s1,
            s2,
            phantom: PhantomData,
        }
    }
}
impl<T, S1: Set<T>, S2: Set<T>> Set<T> for Union<T, S1, S2> {
    fn contains(&self, element: &T) -> bool {
        todo!()
    }
}

// INTERSECTION

/// The intersection of two sets (i.e. the set containing elements in s1 and s2)
///
/// For example:
/// Let A = {0, 1}
/// Let B = {1, 2}
///
/// A intersection B = {1}
pub struct Intersection<T, S1: Set<T>, S2: Set<T>> {
    s1: S1,
    s2: S2,
    phantom: PhantomData<T>, // See what happens if you remove this
}
impl<T, S1: Set<T>, S2: Set<T>> Intersection<T, S1, S2> {
    fn new(s1: S1, s2: S2) -> Self {
        Self {
            s1,
            s2,
            phantom: PhantomData,
        }
    }
}
impl<T, S1: Set<T>, S2: Set<T>> Set<T> for Intersection<T, S1, S2> {
    fn contains(&self, element: &T) -> bool {
        todo!()
    }
}

// DIFFERENCE

/// The difference of two sets (i.e. the set containing elements in s1 and not s2)
///
/// For example:
/// Let A = {0, 1}
/// Let B = {1, 2}
///
/// A minus B = {0}
pub struct Difference<T, S1: Set<T>, S2: Set<T>> {
    s1: S1,
    s2: S2,
    phantom: PhantomData<T>, // See what happens if you remove this
}
impl<T, S1: Set<T>, S2: Set<T>> Difference<T, S1, S2> {
    fn new(s1: S1, s2: S2) -> Self {
        Self {
            s1,
            s2,
            phantom: PhantomData,
        }
    }
}
impl<T, S1: Set<T>, S2: Set<T>> Set<T> for Difference<T, S1, S2> {
    fn contains(&self, element: &T) -> bool {
        todo!()
    }
}

// SYMMETRIC DIFFERENCE

/// The symmetric difference of two sets (i.e. the set containing elements in exatly one of s1 or s2)
///
/// For example:
///   Let A = {0, 1}
///   Let B = {1, 2}
///
///   A symmetric difference B = {0, 2}
pub struct SymmetricDifference<T, S1: Set<T>, S2: Set<T>> {
    s1: S1,
    s2: S2,
    phantom: PhantomData<T>, // See what happens if you remove this
}
impl<T, S1: Set<T>, S2: Set<T>> SymmetricDifference<T, S1, S2> {
    fn new(s1: S1, s2: S2) -> Self {
        Self {
            s1,
            s2,
            phantom: PhantomData,
        }
    }
}
impl<T, S1: Set<T>, S2: Set<T>> Set<T> for SymmetricDifference<T, S1, S2> {
    fn contains(&self, element: &T) -> bool {
        todo!()
    }
}

// CARTESIAN PRODUCT

/// The cartesian product of two sets (i.e. the set containing elements (x in s1, y in s2))
/// Note that here, the types of elements in the two sets may be different
///
/// For example:
/// Let A = {0, 1}
/// Let B = {x, y, z}
///
/// A times B = {(0, x), (0, y), (0, z), (1, x), (1, y), (1, z)}
pub struct CartesianProduct<T, U, S1: Set<T>, S2: Set<U>> {
    s1: S1,
    s2: S2,
    phantom1: PhantomData<T>,
    phantom2: PhantomData<U>,
}
impl<T, U, S1: Set<T>, S2: Set<U>> CartesianProduct<T, U, S1, S2> {
    fn new(s1: S1, s2: S2) -> Self {
        Self {
            s1,
            s2,
            phantom1: PhantomData,
            phantom2: PhantomData,
        }
    }
}
impl<T, U, S1: Set<T>, S2: Set<U>> Set<(T, U)> for CartesianProduct<T, U, S1, S2> {
    fn contains(&self, element: &(T, U)) -> bool {
        todo!()
    }
}
