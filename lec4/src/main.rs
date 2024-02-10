pub mod set;

// Feel free to experiment with how sets work here
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::set::*;

    #[test]
    fn test_empty_set_contains_no_elements() {
        let s = Empty;

        assert!(!s.contains(&0));
        assert!(!s.contains(&1));
        assert!(!s.contains(&2));
        assert!(!s.contains(&3));
        assert!(!s.contains(&4));
        assert!(!s.contains(&5));
    }

    #[test]
    fn test_singleton_set_contains_one_element() {
        let s = Singleton('d');

        assert!(!s.contains(&'a'));
        assert!(!s.contains(&'b'));
        assert!(!s.contains(&'c'));
        assert!(s.contains(&'d'));
        assert!(!s.contains(&'e'));
        assert!(!s.contains(&'f'));
    }

    #[test]
    fn test_universal_set_contains_all_elements() {
        let s = Universal;

        assert!(s.contains(&0.0));
        assert!(s.contains(&1.0));
        assert!(s.contains(&2.0));
        assert!(s.contains(&3.0));
        assert!(s.contains(&4.0));
        assert!(s.contains(&5.0));
    }

    #[test]
    fn test_vec_set_vec_constructor() {
        let s = VecSet::from_vec(vec!["element1", "element2", "element3"]);

        assert!(s.contains(&"element1"));
        assert!(s.contains(&"element2"));
        assert!(s.contains(&"element3"));
        assert!(!s.contains(&"element4"));
    }

    #[test]
    fn test_vec_set_slice_constructor() {
        let s = VecSet::from_slice(&["element1", "element2", "element3"]);

        println!("test");

        assert!(s.contains(&"element1"));
        assert!(s.contains(&"element2"));
        assert!(s.contains(&"element3"));
        assert!(!s.contains(&"element4"));
    }

    #[test]
    fn test_union() {
        let s1 = VecSet::from_slice(&[0, 1, 2]);
        let s2 = VecSet::from_slice(&[2, 3, 4]);

        let u = Union::new(s1, s2);

        assert!(u.contains(&0));
        assert!(u.contains(&1));
        assert!(u.contains(&2));
        assert!(u.contains(&3));
        assert!(u.contains(&4));
        assert!(!u.contains(&5));
    }

    #[test]
    fn test_intersection() {
        let s1 = VecSet::from_slice(&[0, 1, 2]);
        let s2 = VecSet::from_slice(&[2, 3, 4]);

        let u = Intersection::new(s1, s2);

        assert!(!u.contains(&0));
        assert!(!u.contains(&1));
        assert!(u.contains(&2));
        assert!(!u.contains(&3));
        assert!(!u.contains(&4));
        assert!(!u.contains(&5));
    }

    #[test]
    fn test_difference() {
        let s1 = VecSet::from_slice(&[0, 1, 2]);
        let s2 = VecSet::from_slice(&[2, 3, 4]);

        let u = Difference::new(s1, s2);

        assert!(u.contains(&0));
        assert!(u.contains(&1));
        assert!(!u.contains(&2));
        assert!(!u.contains(&3));
        assert!(!u.contains(&4));
        assert!(!u.contains(&5));
    }

    #[test]
    fn test_symmetric_difference() {
        let s1 = VecSet::from_slice(&[0, 1, 2]);
        let s2 = VecSet::from_slice(&[2, 3, 4]);

        let u = SymmetricDifference::new(s1, s2);

        assert!(u.contains(&0));
        assert!(u.contains(&1));
        assert!(!u.contains(&2));
        assert!(u.contains(&3));
        assert!(u.contains(&4));
        assert!(!u.contains(&5));
    }

    #[test]
    fn test_cartesian_product() {
        let s1 = VecSet::from_slice(&[0, 1, 2]);
        let s2 = VecSet::from_slice(&[2, 3, 4]);

        let u = CartesianProduct::new(s1, s2);

        assert!(u.contains(&(0, 2)));
        assert!(u.contains(&(0, 3)));
        assert!(u.contains(&(0, 4)));

        assert!(u.contains(&(1, 2)));
        assert!(u.contains(&(1, 3)));
        assert!(u.contains(&(1, 4)));

        assert!(u.contains(&(2, 2)));
        assert!(u.contains(&(2, 3)));
        assert!(u.contains(&(2, 4)));

        assert!(!u.contains(&(0, 0)));
        assert!(!u.contains(&(1, 1)));
        assert!(u.contains(&(2, 2)));
        assert!(!u.contains(&(3, 3)));
        assert!(!u.contains(&(4, 4)));

        assert!(!u.contains(&(0, 5)));
        assert!(!u.contains(&(1, 5)));
        assert!(!u.contains(&(2, 5)));
        assert!(!u.contains(&(3, 5)));
        assert!(!u.contains(&(4, 5)));
        assert!(!u.contains(&(5, 5)));
    }
}
