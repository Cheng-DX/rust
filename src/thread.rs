#[path = "./utils.rs"]
mod utils;
use std::thread;
use std::time::Duration;
use utils::print_utils::run_tests;

pub mod use_thread {
    use std::{thread, time::Duration};

    use super::run_tests;

    fn spawn() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("For number {i} form spawn thread");
                thread::sleep(Duration::from_millis(i * 10));
            }
        });

        for i in 1..5 {
            println!("Number {i} from main");
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn wait() {
        let handler = thread::spawn(|| {
            for i in 1..10 {
                println!("For number {i} form wait thread");
                thread::sleep(Duration::from_millis(i * 10));
            }
        });

        handler.join().unwrap(); // blocked

        for i in 1..5 {
            println!("Number {i} from main");
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn move_vars() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("{:?}", v);
        });
        handle.join().unwrap();
    }

    fn barrier() {}

    pub fn test() {
        run_tests(vec![spawn, wait, move_vars], "#");
    }
}
