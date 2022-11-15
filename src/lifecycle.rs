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

    pub fn test() {
        case1();
        let mut map = HashMap::new();
        map.insert("i1", 8.0);
        map.insert("i2", 9.0);
        let r = get_default(&mut map, "9");
        println!("{r}");
    }
}
