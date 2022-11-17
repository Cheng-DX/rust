#[path = "./utils.rs"]
mod utils;
use utils::print_utils;

pub mod new_type {
    fn struct_review() {
        #[derive(Debug)]
        struct Point<T>(T, T, T);
        let position = Point(0, 0, 0);
        println!("{:?}", position);
    }

    fn wrapper_types() {
        struct Wrapper<'a>(Vec<&'a str>);

        use std::fmt::{self, Display, Formatter};
        impl<'a> Display for Wrapper<'a> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let v = vec!["a", "b", "c"];
        let w = Wrapper(v);
        println!("{}", w);
    }
    fn type_alias() {
        type Meters = i32;

        fn sum(m1: Meters, m2: Meters) -> Meters {
            m1 + m2
        }

        println!("{}", sum(19, 10));

        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));
        f();
    }

    fn never_return() {
        let i = 1;
        let v = match i {
            0..=3 => i,
            _ => panic!("i is not in range"),
        };
        println!("{}", v);
    }

    use super::print_utils::run_tests;
    pub fn test() {
        run_tests(
            vec![struct_review, wrapper_types, type_alias, never_return],
            "~",
        );
    }
}

pub mod sized_dst {

    fn create_dst() {
        let array = [123; 10];
        println!("array: {:?}", array);
    }

    fn box_str() {
        let s: Box<str> = "Hello World".into();
        println!("s: {} size: {}", s, s.len());
    }

    use super::print_utils::run_tests;
    pub fn test() {
        run_tests(vec![create_dst, box_str], "~");
    }
}

pub mod enum_int {
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    #[derive(FromPrimitive)]
    #[repr(i32)]
    enum Color {
        Red = 1,
        Green,
        Blue,
    }

    fn three_party_lib() {
        let color = 2;
        match FromPrimitive::from_i32(color) {
            Some(Color::Red) => println!("red"),
            Some(Color::Green) => println!("green"),
            Some(Color::Blue) => println!("blue"),
            None => println!("unknown"),
        }
    }

    fn try_from() {
        impl TryFrom<i32> for Color {
            type Error = String;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    x if x == Color::Red as i32 => Ok(Color::Red),
                    x if x == Color::Green as i32 => Ok(Color::Green),
                    x if x == Color::Blue as i32 => Ok(Color::Blue),
                    _ => Err(String::from("unknown")),
                }
            }
        }

        let color = 1;
        match color.try_into() {
            Ok(Color::Red) => println!("red"),
            Ok(Color::Green) => println!("green"),
            Ok(Color::Blue) => println!("blue"),
            Err(e) => println!("{}", e),
        }
    }

    fn transmute() {
        let color = 3 as i32;
        let result: Color = unsafe { std::mem::transmute(color) };

        match result {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
        }
    }
    use super::print_utils::run_tests;
    pub fn test() {
        run_tests(vec![three_party_lib, try_from, transmute], "~");
    }
}
