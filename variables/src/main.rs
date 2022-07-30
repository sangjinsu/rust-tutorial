fn main() {
    let mut x = 5;
    println!("x의 값: {}", x);

    x = 6;
    println!("x의 값: {}", x);

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("number={}", number);

    for i in 0..5 {
        println!("{}", i);
    }

    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    let s1 = String::from("hello");
    take_ownership(s1);

    let x = 5;

    make_copy(x);

    println!("{} {}", s1, x);
}

fn take_ownership(some_string: String){
    println!("{}", some_string);
}

fn make_copy(some_integer: i32){
    println!("{}", some_integer);
}