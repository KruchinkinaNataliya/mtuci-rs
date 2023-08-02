// use tokio::sync::oneshot;
//
// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();
//
//     tokio::spawn(async {
//         let _ = tx1.send("one");
//     });
//
//     tokio::spawn(async {
//         let _ = tx2.send("two");
//     });
//
//     tokio::select! {
//         val = rx1 => {
//             println!("rx1 completed first with {:?}", val);
//         }
//         val = rx2 => {
//             println!("rx2 completed first with {:?}", val);
//         }
//     }
// }

// use tokio::sync::oneshot;
//
// async fn some_operation() -> String {
//     // Compute value here
// }
//
// #[tokio::main]
// async fn main() {
//     let (mut tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();
//
//     tokio::spawn(async {
//         // Select on the operation and the oneshot's
//         // `closed()` notification.
//         tokio::select! {
//             val = some_operation() => {
//                 let _ = tx1.send(val);
//             }
//             _ = tx1.closed() => {
//                 // `some_operation()` is canceled, the
//                 // task completes and `tx1` is dropped.
//             }
//         }
//     });
//
//     tokio::spawn(async {
//         let _ = tx2.send("two");
//     });
//
//     tokio::select! {
//         val = rx1 => {
//             println!("rx1 completed first with {:?}", val);
//         }
//         val = rx2 => {
//             println!("rx2 completed first with {:?}", val);
//         }
//     }
// }

async fn action(input: Option<i32>) -> Option<String> {
    // If the input is `None`, return `None`.
    // This could also be written as `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    // async logic here
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // `.set` is a method on `Pin`.
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}

