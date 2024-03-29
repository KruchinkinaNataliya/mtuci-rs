// use tokio::net::{TcpListener, TcpStream};
// use mini_redis::{Connection, Frame};
//
// #[tokio::main]
// async fn main() {
//     // Bind the listener to the address
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
//
//     loop {
//         // The second item contains the IP and port of the new connection.
//         let (socket, _) = listener.accept().await.unwrap();
//         process(socket).await;
//     }
// }
//
// async fn process(socket: TcpStream) {
//     // The `Connection` lets us read/write redis **frames** instead of
//     // byte streams. The `Connection` type is defined by mini-redis.
//     let mut connection = Connection::new(socket);
//
//     if let Some(frame) = connection.read_frame().await.unwrap() {
//         println!("GOT: {:?}", frame);
//
//         // Respond with an error
//         let response = Frame::Error("unimplemented".to_string());
//         connection.write_frame(&response).await.unwrap();
//     }
// }



// use tokio::net::TcpListener;
//
// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
//
//     loop {
//         let (socket, _) = listener.accept().await.unwrap();
//         // A new task is spawned for each inbound socket. The socket is
//         // moved to the new task and processed there.
//         tokio::spawn(async move {
//             process(socket).await;
//         });
//     }
// }


// #[tokio::main]
// async fn main() {
//     let handle = tokio::spawn(async {
//         // Do some async work
//         "return value"
//     });
//
//     // Do some other work
//
//     let out = handle.await.unwrap();
//     println!("GOT {}", out);
// }

use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}

// use tokio::net::TcpStream;
// use mini_redis::{Connection, Frame};
//
// async fn process(socket: TcpStream) {
//     use mini_redis::Command::{self, Get, Set};
//     use std::collections::HashMap;
//
//     // A hashmap is used to store data
//     let mut db = HashMap::new();
//
//     // Connection, provided by `mini-redis`, handles parsing frames from
//     // the socket
//     let mut connection = Connection::new(socket);
//
//     // Use `read_frame` to receive a command from the connection.
//     while let Some(frame) = connection.read_frame().await.unwrap() {
//         let response = match Command::from_frame(frame).unwrap() {
//             Set(cmd) => {
//                 // The value is stored as `Vec<u8>`
//                 db.insert(cmd.key().to_string(), cmd.value().to_vec());
//                 Frame::Simple("OK".to_string())
//             }
//             Get(cmd) => {
//                 if let Some(value) = db.get(cmd.key()) {
//                     // `Frame::Bulk` expects data to be of type `Bytes`. This
//                     // type will be covered later in the tutorial. For now,
//                     // `&Vec<u8>` is converted to `Bytes` using `into()`.
//                     Frame::Bulk(value.clone().into())
//                 } else {
//                     Frame::Null
//                 }
//             }
//             cmd => panic!("unimplemented {:?}", cmd),
//         };
//
//         // Write the response to the client
//         connection.write_frame(&response).await.unwrap();
//     }
// }