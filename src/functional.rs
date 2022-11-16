pub mod closure {
    use crate::utils::print_utils::run_tests;
    use std::collections::HashMap;
    use std::ops::Range;

    pub fn basic() {
        let sum = |a, b| a + b;
        println!("Sum: {}", sum(1, 2));
    }

    pub fn cache() {
        struct Cacher<T, V: Copy>
        where
            T: Fn(V) -> V,
        {
            query: T,
            value: Option<V>,
        }

        impl<T, V: Copy> Cacher<T, V>
        where
            T: Fn(V) -> V,
        {
            fn new(query: T) -> Cacher<T, V> {
                Cacher { query, value: None }
            }

            fn query_value(&mut self, arg: V) -> V {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.query)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

        let mut map = HashMap::new();
        map.insert("1", "i1");
        map.insert("2", "i2");

        let query_map = |key: &str| -> &str {
            match map.get(&key) {
                Some(v) => v,
                None => "Default",
            }
        };

        let mut cacher = Cacher::new(query_map);
        cacher.value = Some("i3");
        vec!["1", "10", "10"].into_iter().for_each(|key| {
            println!("{} {} {:?}", key, cacher.query_value(key), cacher.value);
        });
    }

    pub fn mode() {
        fn fn_once<'a, F>(func: F)
        where
            F: FnOnce(i32) -> &'a str + Copy,
        {
            println!("{}", func(1));
            println!("{}", func(10));
        }

        let x = 10;
        fn_once(|y| if y == x { "Equal" } else { "NotEqual" });

        let mut s = String::new();
        let mut update_string = |str| s.push_str(str);
        update_string("hello");
        println!("{:?}", s);

        let s = String::from("hello");
        let update_string = move || println!("{}", s);
        update_string();
    }

    pub fn as_return() {
        fn create_adder(base: i32) -> impl FnMut(i32) -> i32 {
            let mut value = base;
            move |x| {
                value += x;
                value
            }
        }

        fn sum(range: Range<i32>) -> i32 {
            let mut adder = create_adder(0);
            for i in range {
                adder(i);
            }
            adder(0)
        }
        println!("{}", sum(1..101));
    }

    pub fn test() {
        run_tests(vec![basic, cache, mode, as_return], "-");
    }
}

pub mod iterator {
    pub fn test() {}
}
