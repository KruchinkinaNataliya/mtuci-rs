// //! # Art
// //!
// //! A library for modeling artistic concepts.
//
// pub mod kinds {
//     /// The primary colors according to the RYB color model.
//     pub enum PrimaryColor {
//         Red,
//         Yellow,
//         Blue,
//     }
//
//     /// The secondary colors according to the RYB color model.
//     pub enum SecondaryColor {
//         Orange,
//         Green,
//         Purple,
//     }
// }
//
// pub mod utils {
//     use crate::kinds::*;
//
//     /// Combines two primary colors in equal amounts to create
//     /// a secondary color.
//     pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
//         // --snip--
//         unimplemented!();
//     }
// } // Библиотека art с элементами, организованными в модули kinds и utils

// use art::kinds::PrimaryColor;
// use art::utils::mix;
//
// fn main() {
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// } // Крейт использующий элементы из крейта art с экспортированной внутренней структурой

// //! # Art
// //!
// //! A library for modeling artistic concepts.
//
// pub use self::kinds::PrimaryColor;
// pub use self::kinds::SecondaryColor;
// pub use self::utils::mix;
//
// pub mod kinds {
//     // --snip--
//     /// The primary colors according to the RYB color model.
//     pub enum PrimaryColor {
//         Red,
//         Yellow,
//         Blue,
//     }
//
//     /// The secondary colors according to the RYB color model.
//     pub enum SecondaryColor {
//         Orange,
//         Green,
//         Purple,
//     }
// }
//
// pub mod utils {
//     // --snip--
//     use crate::kinds::*;
//
//     /// Combines two primary colors in equal amounts to create
//     /// a secondary color.
//     pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
//         SecondaryColor::Orange
//     }
// } // Добавление операторов pub use для реэкспорта элементов

// use art::mix;
// use art::PrimaryColor;
//
// fn main() {
//     // --snip--
//     let red = PrimaryColor::Red;
//     let yellow = PrimaryColor::Yellow;
//     mix(red, yellow);
// } // Программа, использующая реэкспортированные элементы из крейта art




pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
} // В качестве ещё одного улучшения добавили тест функции add_one::add_one в add_one:
