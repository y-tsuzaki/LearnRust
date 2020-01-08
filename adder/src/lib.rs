#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
    //こ の テ ス ト を 失 敗 さ せ る
    panic!("Make this test fail");
    }
}
