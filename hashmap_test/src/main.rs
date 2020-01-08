
use std::collections::HashMap;
    


fn main() {
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_nameとfield_valueはHashMapにムーブされるのでもう使えない

    // println!("{}", field_name); // ERROR!
}
