// check out the derive_macro crate to see how we made this!
use derive_macro::Display;
// check out the function_like_macro crate to see how we made this!
use function_like_macro::add;

// just like the derives that std provides
#[derive(Display)]
struct Foo{
    x: i32,
    y: u32,
}

fn main() {
    let v = Foo {
        x: -42,
        y: 17
    };
    // we made a custom Display impl in only one line of code! (plus all the code in the macro crate)
    println!("{}", v);

    // this can take any number of arguments, which a function can't do
    let x = add!(1 2 3 4);
    assert_eq!(x, 10);
}
