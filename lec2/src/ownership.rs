fn question_1() {
    let mut a = String::from("CIS");
    let mut b = a;

    b += " 1905";

    println!("a = {}, b = {}", a, b);
}

fn question_2() {
    let x = String::from("CIS");

    change(x);

    println!("x = {}", x);
}

fn change(mut x: String) {
    x += " 1905";
}

fn question_3() {
    let x: &i32 = helper();
    println!("x = {}", x)
}

fn helper<'a>() -> &'a i32 {
    let x = 10;
    &x
}

fn main() {
    question_1();
    question_2();
    question_3();
}
