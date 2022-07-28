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
}
