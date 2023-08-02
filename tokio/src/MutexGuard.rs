// use std::sync::{Mutex, MutexGuard};
//
// async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
//     let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
//     *lock += 1;
//
//     do_something_async().await;
// } // lock goes out of scope here //не работает

// This works!
async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}

// // Реструктурируйте свой код, чтобы не удерживать блокировку в .await
// use std::sync::Mutex;
//
// struct CanIncrement {
//     mutex: Mutex<i32>,
// }
// impl CanIncrement {
//     // This function is not marked async.
//     fn increment(&self) {
//         let mut lock = self.mutex.lock().unwrap();
//         *lock += 1;
//     }
// }
//
// async fn increment_and_do_stuff(can_incr: &CanIncrement) {
//     can_incr.increment();
//     do_something_async().await;
// }