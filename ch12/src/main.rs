// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     dbg!(args);
// } // Собираем аргументы командной строки в вектор и выводим их на печать

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
} // Создание переменных для хранения значений аргументов искомой подстроки и пути к файлу
