// fn main() {
//     let v: Vec<i32> = Vec::new();
// } //создание нового вектора

// fn main() {
//     let mut v = Vec::new();
//
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
// } // Использование метода push для добавления значений в вектор

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
} //  Использование синтаксиса индексации и метода get для доступа к элементу в векторе

// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }
//
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// } // Определение enum для хранения значений разных типов в одном векторе

// fn main() {
//     {
//         let v = vec![1, 2, 3, 4];
//
//         // do stuff with v
//     } // <- v goes out of scope and is freed here
// } // Показано как удаляется вектор и его элементы



