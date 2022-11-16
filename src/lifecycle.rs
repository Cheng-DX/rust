#[path = "./utils.rs"]
mod utils;

use utils::print_utils::run_tests;

pub mod basic {
    fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
        if first.len() > second.len() {
            first
        } else {
            second
        }
    }

    pub fn test() {
        let first = "hello";
        let r;
        {
            let second = "world";
            r = longest(first, second);
        }
        println!("{r}");
    }
}

pub mod struct_in {
    struct Button<'a> {
        name: &'a str,
    }

    impl<'a> Button<'a> {
        fn render(&self) -> String {
            format!("|{}|", self.name)
        }
    }

    pub fn test() {
        let name = "CLICK";
        let button = Button { name };
        std::mem::drop(name);
        println!("{}", button.name);
        println!("{}", button.render());
    }
}
pub mod advanced {
    #[derive(Debug)]
    struct Foo;

    impl Foo {
        fn mutate_and_share(&mut self) -> &Self {
            &*self
        }
        fn share(&self) {}
    }

    fn case1() {
        let mut foo = Foo;
        let loan = foo.mutate_and_share();
        println!("{:?}", loan);
        foo.share();
    }

    use std::collections::HashMap;
    use std::hash::Hash;
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        match map.get(&key) {
            Some(_) => map.get_mut(&key).expect("Unexpected error"),
            None => {
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }

    pub fn lifetime_annotation() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a: 'b, 'b> ImportantExcerpt<'a> {
            fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
                println!("Attention please: {}", announcement);
                self.part // should be 'a
                          // If we want to return 'b, we need to guarantee that 'a will live longer than 'b
            }
        }

        let i = ImportantExcerpt { part: "part" };
        i.announce_and_return_part("announcement");
    }

    pub fn closure<'a>() {
        pub fn fn_elision(x: &i32) -> &i32 {
            x
        }
        // let closure_slision = |x: &'a i32| -> &'a i32 { x };

        let num = 5;
        // closure_slision(&num);
        fn_elision(&num);
    }

    use super::run_tests;
    pub fn reborrow() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Point {
            fn move_to(&mut self, x: i32, y: i32) {
                self.x = x;
                self.y = y;
            }
        }
        let mut p = Point { x: 0, y: 0 };
        let r = &mut p;

        let rr: &Point = &*r; // reborrow occurs  -----------------+ rr lifetime starts

        // use these in rr's scope ->
        // println!("{:?}", p);      // 1. Disallowed, cause p is a mutable reference
        // println!("{:?}", r);      // 2. Allowed, cause r is an immmutable reference
        // println!("{:?}", &mut r); // 3. Disallowed, cause &mut r is mmutable

        println!("{:?}", rr); // last use of `rr` occurs here -----+ rr lifetime ends

        r.move_to(10, 10);
        println!("{:?}", r);
    }

    pub fn sample() {
        struct Manager<'a> {
            text: &'a str,
        }
        struct Interface<'a: 'b, 'b> {
            manager: &'b mut Manager<'a>,
        }

        impl<'a, 'b> Interface<'a, 'b> {
            pub fn noop(self) {
                println!("interface consumed {}", self.manager.text);
            }
        }

        struct List<'a> {
            manager: Manager<'a>,
        }

        impl<'a: 'b, 'b> List<'a> {
            pub fn get_interface(&'b mut self) -> Interface<'a, 'b> {
                Interface {
                    manager: &mut self.manager,
                }
            }
        }

        fn run() {
            let mut list = List {
                manager: Manager { text: "hello" },
            };
            list.get_interface().noop();
            println!("Interface should be dropped here and the borrow released");
            use_list(&list);
        }

        fn use_list(list: &List) {
            println!("{}", list.manager.text);
        }

        run();
    }

    use std::{slice::from_raw_parts, str::from_utf8_unchecked};
    pub fn t_static() {
        fn get_memory_location() -> (usize, usize) {
            let string = "Hello World";
            let p = string.as_ptr() as usize;
            let len = string.len();
            (p, len)
        }

        fn get_str(p: usize, len: usize) -> &'static str {
            unsafe { from_utf8_unchecked(from_raw_parts(p as *const u8, len)) }
        }

        fn run() {
            let (pointer, length) = get_memory_location();
            let message = get_str(pointer, length);
            println!(
                "The {} bytes at 0x{:X} stored: {}",
                length, pointer, message
            );
            // segmentation fault
            // let message = get_str(1000, 10);
            // println!("{}", message);
        }

        run();
    }

    pub fn test() {
        let mut map = HashMap::new();
        map.insert("i1", 8.0);
        map.insert("i2", 9.0);
        let r = get_default(&mut map, "9");
        println!("{r}");

        run_tests(
            vec![
                case1,
                lifetime_annotation,
                closure,
                reborrow,
                sample,
                t_static,
            ],
            "-",
        );
    }
}
