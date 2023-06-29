fn main() {

//     Параметры функции
//     fn main() {
//     another_function(5);
// }
//
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

//     При определении нескольких параметров, разделяйте объявления параметров запятыми, как показано ниже:
// fn main() {
//     print_labeled_measurement(5, 'h');
// }
//
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

//     Операторы и выражения
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };
//
//     println!("The value of y is: {y}");
// }
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

// Функции с возвращаемыми значениями
// fn five() -> i32 {
//     5
// }
//
// fn main() {
//     let x = five();
//
//     println!("The value of x is: {x}");
// }
fn plus_one(x: i32) -> i32 {
    x + 1
}
