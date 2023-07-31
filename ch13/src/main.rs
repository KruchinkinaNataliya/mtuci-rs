#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
} // Определение и вызов замыкания, которое захватывает неизменяемую ссылку

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//
//     let mut borrows_mutably = || list.push(7);
//
//     borrows_mutably();
//     println!("After calling closure: {:?}", list);
// } // Определение и вызов замыкания, захватывающего изменяемую ссылку

// use std::env;
// use std::error::Error;
// use std::fs;
//
// pub struct Config {
//     pub query: String,
//     pub file_path: String,
//     pub ignore_case: bool,
// }
//
// impl Config {
//     pub fn build(
//         mut args: impl Iterator<Item = String>,
//     ) -> Result<Config, &'static str> {
//         args.next();
//
//         let query = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a query string"),
//         };
//
//         let file_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file path"),
//         };
//
//         let ignore_case = env::var("IGNORE_CASE").is_ok();
//
//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//         })
//     }
// }
//
// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//
//     let results = if config.ignore_case {
//         search_case_insensitive(&config.query, &contents)
//     } else {
//         search(&config.query, &contents)
//     };
//
//     for line in results {
//         println!("{line}");
//     }
//
//     Ok(())
// }
//
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }
//
// pub fn search_case_insensitive<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();
//
//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }
//
//     results
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";
//
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
//
//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";
//
//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// } // Использование методов адаптера итератора в реализации функции search
