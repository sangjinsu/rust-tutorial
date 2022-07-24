use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자 맞추기");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("맞춰야 할 숫자 {}", secret_number);

    loop {
        println!("정답 입력!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력 값 {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 적습니다"),
            Ordering::Greater => println!("입력한 숫자가 큽니다"),
            Ordering::Equal => {
                println!("정답");
                break;
            }
        }
    }
}
