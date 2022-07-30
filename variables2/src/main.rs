fn main() {
    // 참조에 대한 규칙
    // 1. 한 시점에 코드는 한 가변 참조 또는 여러 개의 불번 참조를 생성할 수 있지만 둘 모두 생성은 불가
    // 2. 참조는 한상 유효해야 한다
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    println!("{:p} {:p}", &s1, &s2);

    let s3 = takes_and_give_ownership(s2);

    let (s4, len) = calculate_length(s1);

    println!("{} {}", s4, len);

    println!("{:p} {:p}", &s3, &s4);

    let mut str = String::from("hello world");
    let word = first_word(&str);
    str.clear();
    println!("{}", word);

}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_give_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}