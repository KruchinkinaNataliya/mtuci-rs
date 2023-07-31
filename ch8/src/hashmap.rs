// fn main() {
//     use std::collections::HashMap;
//
//     let mut scores = HashMap::new();
//
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
// } // Создание новой хеш-карты и вставка в неё пары ключей и значений

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
} // Доступ к данным в HashMap

