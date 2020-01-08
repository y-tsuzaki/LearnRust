fn main() {
    let number = 6;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }

    // ifは式だから三項演算子みたいな書き方ができるんだなあ
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    // numberの値は、{}です
    println!("The value of number is: {}", number);
}