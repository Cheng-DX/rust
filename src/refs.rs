#[path = "./utils.rs"]
mod utils;
use utils::print_utils::run_tests;

pub mod weak {
    use std::cell::RefCell;
    use std::rc::Rc;
    fn r() {
        use List::{Cons, Nil};

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<(i32, &RefCell<Rc<List>>)> {
                match self {
                    Cons(value, item) => Some((*value, item)),
                    Nil => None,
                }
            }
        }

        let last = RefCell::new(Rc::new(Cons(10, RefCell::new(Rc::new(Nil)))));
        let a = Rc::new(Cons(5, last));

        let b = Cons(3, RefCell::new(Rc::clone(&a)));
        let c = Cons(4, RefCell::new(Rc::clone(&a)));

        println!("a strong count is {}", Rc::strong_count(&a));
        println!(
            "b.tail is: {:?} \n c.tail is: {:?}",
            b.tail().unwrap(),
            c.tail().unwrap()
        );
    }

    fn grade() {
        let five = Rc::new(5);
        let weak_five = Rc::downgrade(&five);
        let strong_five = weak_five.upgrade();
        println!("{}", strong_five.unwrap());
        drop(five);
        // let strong_five = weak_five.upgrade();
        // println!("{}", strong_five.unwrap());
    }

    use super::run_tests;
    pub fn test() {
        run_tests(vec![r, grade], "-");
    }
}

pub mod self_refed_struct {
    fn use_option() {
        #[derive(Debug)]
        struct Node<'a> {
            value: String,
            ptr: Option<&'a str>,
        }

        let s = String::from("Hello");
        let mut node = Node {
            value: s,
            ptr: None,
        };

        node.ptr = Some(&(node.value));
        println!("{:?}", node);
    }

    fn use_unsafe() {
        #[derive(Debug)]
        struct Node {
            value: String,
            ptr: *const String,
        }

        impl Node {
            fn new(txt: &str) -> Self {
                let s = String::from(txt);
                let mut node = Node {
                    value: s,
                    ptr: std::ptr::null(),
                };
                node.ptr = &node.value;
                node
            }

            fn get_value(&self) -> &str {
                &self.value
            }

            fn get_ptr(&self) -> &String {
                assert!(!self.ptr.is_null(), "Node::ptr is null");
                unsafe { &*(self.ptr) }
            }
        }
        let node = Node::new("From");
        let node2 = Node { ..node };
        println!("{:p} {}", node2.get_ptr(), node2.get_value());
    }

    use std::{borrow::Borrow, pin::Pin, ptr::NonNull};
    fn use_pin() {
        #[derive(Debug)]
        struct Node {
            data: String,
            slice: NonNull<String>,
        }

        impl Node {
            fn new(data: String) -> Pin<Box<Self>> {
                let res = Node {
                    data,
                    slice: NonNull::dangling(),
                };

                let mut boxed = Box::pin(res);
                let slice = NonNull::from(&boxed.data);
                unsafe {
                    let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                    Pin::get_unchecked_mut(mut_ref).slice = slice;
                }
                boxed
            }
        }

        let node = Node::new(String::from("hello"));
        println!("{:?}", node.slice);
    }

    fn use_ouroboros() {
        use ouroboros::self_referencing;

        #[self_referencing]
        struct Node {
            value: String,
            #[borrows(value)]
            ptr: &'this str,
        }

        let node = NodeBuilder {
            value: String::from("use_ouroboros"),
            ptr_builder: |value: &String| value,
        };
        let s = node.borrow();
        println!("{}", s.value);
    }

    use super::run_tests;
    pub fn test() {
        run_tests(vec![use_option, use_unsafe, use_pin, use_ouroboros], "-");
    }
}
