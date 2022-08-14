#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        let result = "hi";
        assert_eq!(result, "hi", "에러 발생")
    }

    #[test]
    #[should_panic]
    fn another2() {
        panic!("패닉")
    }
}
