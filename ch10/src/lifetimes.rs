// fn main() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {}", r);
// } // Попытка использования ссылки, значение которой вышло из области видимости

// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+
// // Аннотация времён жизни переменных r и x, с помощью идентификаторов времени
// // жизни 'a и 'b, соответственно

// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }                         // ----------+
// // Ссылка корректна, так как данные имеют более продолжительное время жизни, чем ссылка на эти данные

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }
//
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// } // Структура, содержащая ссылку, требует аннотации времени жизни

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// Обобщённые типы параметров, ограничения типажей и времена жизни вместе
