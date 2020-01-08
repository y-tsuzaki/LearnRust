use std::fmt::Display;


// whereじゃなくてこうもかける
// fn longest_with_un_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann : T) -> &'a str {}
where T:Display {
fn longest_with_un_announcement<'a, T>(x: &'a str, y: &'a str, ann : T) -> &'a str 
    where T:Display {
    println!("Announcement {}!", ann);
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("hogehoge");
    let b = String::from("piyo");
    let c = String::from("test!");

    let result = longest_with_un_announcement(&a, &b, &c);

    println!("result: {}", result);
}
