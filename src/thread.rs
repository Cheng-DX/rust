#[path = "./utils.rs"]
mod utils;
use utils::print_utils::run_tests;

pub mod use_thread {
    use std::{
        cell::Ref,
        sync::{Arc, Barrier, Condvar, Mutex},
        thread::{self, LocalKey},
        time::Duration,
    };

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

    fn barrier() {
        let mut handlers = Vec::with_capacity(6);
        let barrier = Arc::new(Barrier::new(6));

        for _ in 0..6 {
            let b = barrier.clone();
            handlers.push(thread::spawn(move || {
                println!("before await");
                thread::sleep(Duration::from_millis(100));
                b.wait();
                println!("after awiat");
            }));
        }
        for handler in handlers {
            handler.join().unwrap();
        }
    }

    use std::cell::RefCell;
    fn local_vars() {
        thread_local! {
            static FOO: RefCell<u32> = RefCell::new(1)
        };

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        let t = thread::spawn(move || {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            })
        });

        t.join().unwrap();

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });

        struct Item;
        impl Item {
            thread_local! {
                static NAME: RefCell<usize> = RefCell::new(0);
            }
        }
        Item::NAME.with(|x| println!("{:?}", x));

        thread_local! {
            static AGE: RefCell<usize> = RefCell::new(12);
        }
        struct Bar {
            age: &'static LocalKey<RefCell<usize>>,
        }
        impl Bar {
            fn new() -> Self {
                Self { age: &AGE }
            }
        }

        let bar = Bar::new();
        bar.age.with(|age| println!("{:?}", age));
    }

    fn mutex() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pairs = pair.clone();
    }

    pub fn test() {
        run_tests(
            vec![spawn, wait, move_vars, barrier, local_vars, mutex],
            "#",
        );
    }
}
