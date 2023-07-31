// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// use crate::List::{Cons, Nil};
//
// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// } // Определение List, которое использует Box<T> для того, чтобы иметь вычисляемый размер

// fn main() {
//     let x = 5;
//     let y = &x;
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// } // Использование оператора разыменования для следования по ссылке к значению i32

// fn main() {
//     let x = 5;
//     let y = Box::new(x);
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// } // Использование оператора разыменования с типом Box<i32>

// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
//
// fn main() {} // Определение типа MyBox<T>

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     println!("CustomSmartPointers created.");
// } // Структура CustomSmartPointer, реализующая типаж Drop, куда мы поместим наш код очистки

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
} // Вызов std::mem::drop для принудительного удаления значения до того, как оно выйдет из области видимости

