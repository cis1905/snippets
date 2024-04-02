#[macro_export]
macro_rules! add {
    // A macro_rules macro is like one big match,
    // except instead of matching patterns of types to values,
    // we're matching patterns of syntax to the part of the program that the macro is reading.

    // Like in regular expressions, * in the input pattern means "look for zero or more elements like this"
    ( $( $x:expr )* ) => {
        {
            let mut total = 0;
            // we can loop over the input by making a syntax that mirrors our pattern match from earlier
            $(
                total += $x;
            )* // * in the output means "do the things we were matching earlier once for each match we had"
            total
        }
    };
}

#[macro_export]
macro_rules! group_apply {
    // like in regular expressions, * means "zero or more elements"
    ( $( $x:expr ),* => $y: expr ) => {
        {
            $(
                $y($x);
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        // now we can use `add!` and add our stuff with no + sign there!
        let result = add!(1 2 3 4);
        assert_eq!(result, 10);
    }

    #[test]
    fn group_apply_works_basics() {
        // now we can use `add!` and add our stuff with no + sign there!
        let mut total = 0;
        let mut f = |x| total += x; 
        group_apply!(1, 2, 3, 4 => f);
        assert_eq!(total, 10);
    }

    #[test]
    fn group_apply_works_variadic_trait() {
        // now we can use `add!` and add our stuff with no + sign there!
        fn print_to_string<T: std::fmt::Debug>(x: T, ){
            println!("printing: {x:?}");
        }
        group_apply!(1, 2, 3, 4 => print_to_string);
    }
}
