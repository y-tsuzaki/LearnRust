
use std::collections::HashMap;


fn main() {
    let vec: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10,3,3,3];

    let mean = get_mean(&vec);

    println!("mean is {}", mean);


    let mean = get_median(&vec);

    println!("median is {}", mean);

    let mode = get_mode(&vec);

    println!("mode is {}", mode);
}

//整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。

fn get_mean(values:&Vec<i32>) -> f64 {
    let mut sum = 0;
    for value in values {
        sum += value;
    }

    sum as f64 / values.len() as f64
}

fn get_median(values:&Vec<i32>) -> i32 {
    let mut sorted = values.clone();
    sorted.sort();

    sorted[ values.len() / 2 ]
}

// うーん、ローカル変数の参照は返せません。みたいな感じで怒られた
//error[E0515]: cannot return value referencing local variable `map`
// 53 |     map.get(&max_count_key)
//    |     ---^^^^^^^^^^^^^^^^^^^^
//    |     |
//    |     returns a value referencing data owned by the current function
//    |     `map` is borrowed here
fn get_mode<'a>(values: &'a Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for value in values {
        let count = map.entry(*value).or_insert(0);
        *count += 1;
    }
 
    let mut max_count: i32 = 0;
    let mut max_count_key: i32 = 0;
    for (key, value) in &map {
        if max_count < *value {
            max_count = *value;
            max_count_key = *key;
        }
    }

    max_count_key
}
