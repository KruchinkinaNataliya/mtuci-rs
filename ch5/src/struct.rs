// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
// } создание экземпляра структуры

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// } Функция build_user, которая принимает email и имя пользователя и возвращает экземпляр User

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
//
// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
// } Использование сокращённой инициализации поля

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
// : Создание нового экземпляра User с использованием некоторых значений из экземпляра user1