# LearnRust
2019年末を利用してRustを学びますよ

# なにからやろうかな

日本語ドキュメントをまとめたもの
https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/README.html

このドキュメントが一番新しいっぽい
https://doc.rust-jp.rs/book/second-edition/

これをやるぞ

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

トレイトの継承で有用な機能を追加する

- strictをprintln!で{}で表示するには`Display`を実装する必要がある
- 基本型は全部Displayを実装してるので表示できた
- Debugを実装すれば`{:?}`で表示できる

- Debugを実装するにはトレイトを呼び出す`#[derive(Debug)]`
- `{:#?}`とすると改行された形で表示される


```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1は{}です
    println!("rect1 is {}", rect1);
}
```

### メソッド記法

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
メソッドの実装はimpl句でstructと同名の指定する。

メソッドの第一引数が`&self`

# "6,.Enumとパターンマッチング"
### Enum
```rust
enum IpAddrKind {
    V4,
    V6,
}
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

rustのenumは、もはや列挙型の範疇を超えてる。
```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```
```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

enumなのに値を代入できるー！ 便利なのはわかるけど、 Enumの概念がわけわからなくなってきた。
これを活用すると他の言語と全然違う実装になりそう。

enumにもimplでメソッド生やせる

####  Optional

nullの開発者はnullのことを「10億ドルの失敗」と言ってる話。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust

let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
使う時はこうする

まだあんまりピンときてない。

## matchフロー制御演算子

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

```rust

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),// わあこうやってEnumがネストするのか 
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 引数に入ってくる
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

### Option<T>とのマッチ
  

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
おお、なんかわかってきた。繋がってきた。nullableな変数ですよということを静的にずっと引き回していける感じか。
Optionだから処理する前に判定することを矯正できるのね。

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // Error
    }
}
```
matchはすべてのEnumのパターンを網羅しないとエラーになる

### _というプレースホルダー

全部網羅するのは大変。_を使って、ifと同じようにelseみたいなことができる
```rust
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

## if let

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

と以下は等価
if let Some(3) = some_u8_value {
    println!("three");
}
```

マッチしたい値が一個の時はif let　でええんやね。
if letってあんまり直感的でない命名な気もするけど。


elseもかけるのか。これはmutchじゃなくてif分だと思えばいいのかな。
```rust

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

# "7.モジュール"

- `cargo new communicator --lib`でモジュールを作る

内部モジュール

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

ファイルを外に出すには、モジュール名でファイルを作って中身を書く。modブロックはいらない。

ファイル名: src/lib.rs
```rust
mod client;
```
 ファイル名: src/client.rs
```rust
fn connect() {
}
```

### modとファイルシステム

サブモジュールは、サブモジュール名.rsと命名する
サブモジュールにサブモジュールがある場合は

```
sub_module/mod.rs
sub_module/sub_sub_module.rs
```
とする

### pubで公開するか制御する

```
pub mod hogehoge;
```
として
```
pub fn hoge(){}
```
となれば外部からアクセスできる

アクセスするときは`extern crate module_name;`で読み込んで
`module_name::sub_module_name::function_name();`で実行

ただし、同一クレート（cargo newでできたフォルダ）以外からアクセスする方法はまだわからない


# 一般的なコレクション

## ベクタ型

同じ型のリストみたいなもん。単方向リストか？

#### 宣言する
```rust
let v: Vec<i32> = Vec::new();
```

#### 代入する
```rust
let v = vec![1, 2, 3];
```

#### データを追加する
```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
```

ベクタがドロップすると、ベクタの中のあたいもドロップする

```rust
{
    let v = vec![1, 2, 3, 4];

    // vで作業をする

} // <- vはここでスコープを抜け、解放される
```

#### ベクタの要素を読む

```

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```
[]でもいけるし、get()でもいける

[]だとRuntimeExceptionだけど
get()だとOption<＆T>だから安全だな

#### 参照されてると変更できない
`同一スコープ上では、可変と不変な参照を同時には存在させられないというルール`のため

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

0番目しか借用してないのにpushできないのは、pushするとメモリの再配置が起こるかもしれないから。


#### Enumを使えば複数の型をvectorに入れられる
```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

なぜならEnumはどんな方になり得るかコンパイラが知ってるから。


## 文字列型

```rust
let mut s = String::new();
```

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

文字リテラルと文字列は違うんだね

### 連結

```rust

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
```

+は`fn add(self, s: &str) -> String {`らしいが、あとで詳しく出てくるだろうからスルー。

```rust

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```
format!はprintln!とほとんど同じ。formatのほうは文字列を返す.


### 添字でアクセス

マルチバイト文字なので添字でアクセするのはよくない（バグる）

chars()で取るのが最適とのこと
```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

## ハッシュマップ

```rust

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```



```

use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
HashMap<_, _> をつけなきゃいけない理由はよくわからない
collect()の返り値の型はHashMapじゃないのか？
何がくるかわからないのにHashMapにしていいの？
_ってなに？ 

### ハッシュマップの値にアクセスする

```rust

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
getで取得する。 返り値はOption<&V>


```

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
forで回す


```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

なかったらいれる
```rust
scores.entry(String::from("Blue")).or_insert(50);
```

古い値に基づいて値を更新する
```rust

use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```
これはちょっとむずいな。 or_insert()はキーに対する可変参照(&mut V)を返すからこう書ける。



















