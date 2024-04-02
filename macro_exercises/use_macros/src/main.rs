use derive_macro::Display;
use function_like_macro::add;

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
    println!("{}", v);

    let x = add!(1 2 3 4);
    assert_eq!(x, 10);
}
