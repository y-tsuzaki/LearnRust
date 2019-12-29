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


# "3.一般的なプログラミングの概念"

## 3.1 変数と可変性

変数と定数の違い
- constは定数しか入れられないが、letはメソッドの返り値など実行時に評価される値を入れることもできる
- letはシャドーイングできる

### シャドーイングとは

おなじ変数名で変数を覆い隠すこと
```
let x = 5;
let x = x + 1;
```
ミュータブルじゃないの？と思うが、mutと違い、覆い隠す時はletを使わなければならないのがちがう

同じ変数名を使い回せるのがメリットとのこと。うーん、メリットなのか？

これはOK（型が文字列から数値に変わっているがletしてるのでOK)
```
let spaces = "   ";
let spaces = spaces.len();
```

これはNG（静的型チェックでエラー）
```
let mut spaces = "   ";
spaces = spaces.len();
```

＃＃ データ型
Rustでは全ての値はデータ型になる。

２つのデータ型
- スカラー型
- 複合型

複数の型が返ってくる可能性がある場合は、型注釈が必要
↓`：u32`の部分
```
let guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！
```

#### スカラー型
整数、浮動小数点数、論理値、文字

- 整数 : i32が基準型、 uはunsigned
- 浮動小数点数 : f6４が基準型 f32もある
- 論理値 : bool
- 文字 : char 

シングルクオートでくくる

#### 複合型

- タプル型 ： `let tup: (i32, f64, u8) = (500, 6.4, 1);`
- 配列型 : `let a = [1, 2, 3, 4, 5];`

### 関数
snake_caseで書く

#### 式について
文は値を返さない. CやJavaなんかは代入式も値を返すがRustでは返さない
```
let x = 1;
```

ブロック{}も式
```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
 x + 1にセミコロンがついてないのが重要。つけないことで、ブロックが文ではなく、式になる。
 
 

```rust
fn five() -> i32 {
    5
}
```
リターン入らない。
セミコロンのない行が式となり、5という式が評価され5という値が返る

### コメント
// しかない

### フロー制御
#### if
普通にif文もあった。

```rust 
// ifは式だから三項演算子みたいな書き方ができるんだなあ
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
```

#### loop
```rust
loop {}
```

#### while
```rust
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
```

### for
`for ... in ... {}`をつかう
```rust
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
```

`(1..4)`で1から4のイテレータ?を作って、rev()で反転して、それをnubmerに食わしてるのか
```
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```


# "4.所有権を理解する"

Rustの最もユニークな機能であり、これのおかげでガベージコレクタなしで安全性担保を行うことができる

## 所有権

#### スタックとヒープ

スタックは、コンパイル時にメモリ領域が決まっているやつが格納される領域。
サイズが決まっているスカラー値や、文字列リテラルなどが格納される。アクセスが早い。

ヒープは、コンパイル時にサイズのわからない、サイズが可変であるデータが格納される

#### 所有権規則
- Rustの各値は、所有者と呼ばれる変数と対応している。
- いかなる時も所有者は一つである。
- 所有者がスコープから外れたら、値は破棄される。

- 変数がスコープから出たらメモリが返還される
- スコープが出た時、rustは特別な関数dropを呼ぶ
### 変数とデータの相互作用法: ムーブ

Rustはシャローコピーでもディープコピーでもなく、　`ムーブ`する
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // ERROR!

### クローン
```:rust

let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2); //OK だが遅い
```

スタックだけで完結するやつは、普通にコピーされる。変数宣言した時点でメモリが確保されているから。
```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

コピーされる型
- あらゆる整数型。u32など。
- 論理値型、bool、trueとfalseという値がある。
- あらゆる浮動小数点型、f64など。
- 文字型、char。
- タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。

### 所有権と関数

何も工夫しないで引数に変数を渡すと、関数のスコープに変数がムーブして、関数のスコープを抜ける時にDropされてしまう。

### 参照と借用

&をつけると参照渡しできる
```
let len = calculate_length(&s1);
```

引数の型は＆Stringのように&がつく
```
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

#### 可変な参照

##### mutキーワードで可変な参照を作れる
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

##### １つの変数に複数の可変な参照は作れない.
これによって、データ競合が発生し得ないようになっている。

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;//Error!
```

##### 普遍な借用があるときも可変な参照を作れない
```
let mut s = String::from("hello");

let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！
```

##### 宙にぶら下がった参照
ダングリングポインタを防止する仕組みがある

```rust
fn dangle() -> &String { // dangleはStringへの参照を返す

    let s = String::from("hello"); // sは新しいString

    &s // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 危険だ
```

この場合は、借用を返すのではなく実態（っていうのか？）を返すのがいい


### スライス

配列の部分的な参照。
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```
この場合aの型は`&[i32]`となる

文字列のスライスは、`&str`、文字列はイレギュラーなんだろうか？

スライスは`&変数名[添字..添字]`で作る
```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

#### 文字列リテラルはスライスである
```rust
let s = "Hello, world!";
```
これは全部を含むスライスだったのだ！

実態はヒープにある


# 構造体を定義し、インスタンス化する
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

構造体のカラム名と一緒なら省略できる
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

