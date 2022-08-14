use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let v = vec![1, 2, 3];
    //
    // v[99];

    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(_e) => panic!("파일 생성 실패")
            },
            _other_error => panic!("파일 열기 실패")
        }
    };
}
