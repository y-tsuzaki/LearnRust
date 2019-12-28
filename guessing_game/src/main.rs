use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed tot read line");

        // match式の例。parse()はResult型を返す。Resultはenum.OkまたはErr値を返す。
        // Ok(), Err()はmatch式のメソッドなのか？
        // まだわからないがそのうちわかるようになるだろう。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You geessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
