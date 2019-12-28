# LearnRust
2019年末を利用してRustを学びますよ

# なにからやろうかな

日本語ドキュメントをまとめたもの
https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/README.html

このドキュメントが一番新しいっぽい
https://doc.rust-jp.rs/book/second-edition/

Rust 2018という言語バージョンがあり、旧Rustから破壊的な更新があるみたいだけど、日本語ドキュメントがないのでスルーする。
基本文法は同じだと思うので。この辺は詰まってから考えよう。

# 概要
- 参考：まえがき
- Rustは、メモリ管理、データ表現、並行性に優れていて、低レベルの繊細な処理が得意な模様
- それぞれ優れた仕組みがあって、楽で安全にそれらのことができる。
- Rustは低レベルな表現にかぎらず、十分に表現力があってエルゴノミック（人間工学に基づいてる）ので、コマンドラインアプリからWebサーバその他色々に使える
- Rustは安全でスピードが早い

# インストール

```
curl https://sh.rustup.rs -sSf | sh

source $HOME/.cargo/env
```

`rustc`でコンパイルできるようになった。
`cargo`というビルドツールも勝手にインストールされている。

## cargoについて

### 今のところの理解

- phpでいうartisanみたいであり、composerみたいでもある感じ。
- プロジェクトを作るとCargo.tomlというファイルができて、これがcomposerのcomposer.jsonとかnpmのpackage.jsonみたいな感じ

### よく使いそうなコマンド
- cargo new my_package_name --bin でプロジェクト作成
- cargo build でビルド
- cargo check で構文チェック(ビルドよりファイル書き出ししない分早い）
- cargo run でビルドアンド実行

# "2. 数当てゲームをプログラムする"

https://doc.rust-jp.rs/book/second-edition/ch02-00-guessing-game-tutorial.html


- 所感
  - 乱数取得の行、何書いてるかわからない
  - match文よくわからない、 if文じゃないのには意味がありそう。これが関数型ってやつなのか？
  - .expect()でエラーを拾ってる？ nullチェック？
  - println!の!ってなんなん
  - intはu32かu64
  - let mutでmutable
  - letでimmutable (phpのconstみたいな感じか。mut明記しないとimmutableなのはphp,java,jsと違って面白い)
  
  
```rust
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
```


