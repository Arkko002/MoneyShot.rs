// TODO Consider using non-async mutex parking_lot, https://tokio.rs/tokio/tutorial/shared-state#restructure-your-code-to-not-hold-the-lock-across-an-await

use parking_lot::Mutex;
use std::sync::Arc;

pub struct DoubleBuffer<T> {
    is_first_read: bool,
    first_buf: Arc<Mutex<T>>,
    second_buf: Arc<Mutex<T>>,
}

// impl<T> DoubleBuffer<T> {
//     pub fn new(value: T) -> Self {
//         Self {
//             is_first_read: true,
//             first_buf: Arc::new(Mutex::new(value)),
//             second_buf: Arc::new(Mutex::new(value)),
//         }
//     }
//
//     pub fn construct_with<F: Fn() -> T>(constructor: F) -> Self {
//         Self {
//             is_first_read: true,
//             first_buf: Arc::new(Mutex::new(constructor())),
//             second_buf: Arc::new(Mutex::new(constructor())),
//         }
//     }
//
//     pub fn read(&self) -> T {
//         match self.is_first_read {
//             true => {
//                 self.is_first_read = !self.is_first_read;
//                 self.first_buf.into_inner()
//             }
//             false => {
//                 self.is_first_read = !self.is_first_read;
//                 self.second_buf.into_inner()
//             }
//         }
//     }
//
//     pub fn write(&self, value: T) -> () {
//         match self.is_first_read {
//             true => self.write_into_buffer(value, self.second_buf),
//             false => self.write_into_buffer(value, self.first_buf),
//         }
//     }
//
//     fn write_into_buffer(&self, value: T, buffer: Arc<Mutex<T>>) -> () {
//         let inner = buffer.lock();
//         *inner = value;
//     }
// }
