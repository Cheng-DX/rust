pub mod map {
    use std::{collections::HashMap, hash::Hash};

    fn from<K: Eq + Hash, V>(keys: Vec<K>, list: Vec<V>) -> HashMap<K, V> {
        keys.into_iter().zip(list.into_iter()).collect()
    }

    fn query<'a, K: Eq + Hash, V>(src: &'a HashMap<K, V>, key: K) -> &'a V {
        src.get(&key).unwrap()
    }

    pub fn test() {
        let strings = vec![
            String::from("item1"),
            String::from("item2"),
            String::from("item3"),
        ];
        let numbers = vec![1, 2, 3, 5];
        let map = from(strings, numbers);
        println!("{:?}", map);
        let key = String::from("item2");
        println!("{key}");
        let value = query(&map, key);
        println!("{value}");
    }
}

pub mod vector {
    pub fn test() {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        println!("The first element is: {first}"); // avaiable
        v.push(6);

        // Wrong
        // println!("The first element is: {}", first);
        // v.push will add a new element to the vector,
        // which may cause the whole vector be removed another memory location.
        // The reference to the first element will be invalid.
    }
}
