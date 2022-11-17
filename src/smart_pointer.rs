#[path = "./utils.rs"]
mod utils;

use utils::print_utils::run_tests;

pub mod box_pointer {
    fn malloc_in_heap() {
        let a = Box::new(3);
        println!("a = {}", a);

        let b = *a;
        println!("b = {}, {}", b, a);

        let arr = [0; 1000];
        let arr_copied = arr;

        println!("arr = {:?}", arr);
        println!("arr_copied = {:?}", arr_copied);

        let box_arr = Box::new([0; 1000]);
        let box_arr_copied = box_arr;
        // println!("box_arr = {:?}", box_arr); // error[E0382]: borrow of moved value: `box_arr`
        println!("box_arr_copied = {:?}", box_arr_copied);
    }

    fn into_sized() {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        } // sized type
    }
    use super::run_tests;
    pub fn test() {
        run_tests(vec![malloc_in_heap, into_sized], "-");
    }
}

pub mod deref {
    fn get_ref() {
        let x = 5;
        let y = &x;

        println!("x = {x}, y = {y}");
    }

    fn my_box() {
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let b = MyBox::new(5);
        println!("b = {}", *b);

        use std::ops::DerefMut;

        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }

    fn deref() {
        let s = String::from("hello");
        use_s(&s);

        fn use_s(s: &str) {
            println!("{}", s);
        }
    }
    use std::ops::Deref;

    use super::run_tests;
    pub fn test() {
        run_tests(vec![get_ref, my_box, deref], "-");
    }
}

pub mod drop {
    fn drop_struct() {
        struct HasDrop1;
        struct HasDrop2;
        impl Drop for HasDrop1 {
            fn drop(&mut self) {
                println!("Dropping HasDrop1!");
            }
        }
        impl Drop for HasDrop2 {
            fn drop(&mut self) {
                println!("Dropping HasDrop2!");
            }
        }
        struct HasTwoDrops {
            one: HasDrop1,
            two: HasDrop2,
        }
        impl Drop for HasTwoDrops {
            fn drop(&mut self) {
                println!("Dropping HasTwoDrops!");
            }
        }

        struct Foo;

        impl Drop for Foo {
            fn drop(&mut self) {
                println!("Dropping Foo!")
            }
        }

        let _x = HasTwoDrops {
            one: HasDrop1,
            two: HasDrop2,
        };
        let _foo = Foo;
        println!("Running!");
    }

    use super::run_tests;
    pub fn test() {
        run_tests(vec![drop_struct], "-");
    }
}

pub mod rc_arc {

    use std::{rc::Rc, sync::Arc, thread};
    fn rc() {
        let a = Rc::new(String::from("Hello World"));
        let b = Rc::clone(&a);

        println!("{} {}", a, b);
        println!("{}", Rc::strong_count(&a));
    }
    fn count() {
        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&b));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    fn arc() {
        let s = Arc::new(String::from("From Arc"));
        for _ in 0..10 {
            let inner = Arc::clone(&s);
            let handle = thread::spawn(move || {
                println!("{inner}");
            });
        }
    }

    use super::run_tests;
    pub fn test() {
        run_tests(vec![rc, count, arc], "-");
    }
}

pub mod cell {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use super::run_tests;

    fn cell() {
        let c = Cell::new("hello");
        let one = c.get();
        c.set("world");
        let two = c.get();
        println!("{one} {two}");
    }

    // fn ref_cell() {
    //     let s = RefCell::new(String::from("RefCell"));
    //     let s2 = s.borrow_mut();
    //     println!("{s2}");

    //     let s1 = s.borrow();
    //     println!("{s1}");
    // }

    fn ref_cell_1() {
        let mut s = String::from("String");
        let s2 = &mut s;
        println!("{s2}");

        let s1 = &s;
        println!("{s1}");
    }

    fn rc_refcell() {
        let s = Rc::new(RefCell::new(String::from("Rc + RefCell")));
        let s1 = s.clone();
        let s2 = s.clone();

        s2.borrow_mut().push_str("TEST");
        println!("{:?}\n{:?}\n{:?}", s, s1, s2);
    }

    pub fn test() {
        run_tests(vec![cell, ref_cell_1, rc_refcell], "-");
    }
}
