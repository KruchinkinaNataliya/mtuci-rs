// fn main() {
//     for i in 1..=100 {
//         if i%15 == 0 {
//             println!("FizzBuzz"); }
//         else {
//             if i%5 == 0 {
//                 println!("Buzz"); }
//             else {
//                 if i%3 == 0 {
//                     println!("Fizz"); }
//                 else {
//                     println!("{i}");}
//             };
//         };
//     };
//  }

fn main() {
    for i in 1..=100 {
        match (i%5 == 0, i%3 == 0) {
            (false, true) =>
                println!("Fizz"),
            (true, false) =>
                println!("Buzz"),
            (true, true) =>
                println!("FizzBuzz"),
            _ =>
                println!("{i}") };
    };
}
