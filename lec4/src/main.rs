pub mod set;

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
}
