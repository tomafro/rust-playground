use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
struct ValueExample {
    value: i64,
    negative: RefCell<Option<i64>>
}

impl ValueExample {
    fn new(value: i64) -> ValueExample {
        let negative = RefCell::new(None);
        ValueExample { value, negative }
    }

    fn negative(&self) -> i64 {
        *self.negative.borrow_mut().get_or_insert_with(|| -self.value )
    }
}

struct HashMapExample {
    cache: RefCell<HashMap<String, i64>>
}

impl HashMapExample {
    fn new() -> HashMapExample {
        let map: HashMap<String, i64> = HashMap::new();
        let cache = RefCell::new(map);
        HashMapExample { cache }
    }

    fn get(&self, key: &str) -> i64 {
        *self.cache.borrow_mut().entry(key.to_owned()).or_insert_with(|| {
            let value: i64 = key.parse().unwrap();
            -value
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_negative_on_demand() {
        let example = ValueExample::new(1);
        assert_eq!(-1, example.negative());
        assert_eq!(-1, example.negative());
    }

    #[test]
    fn hashmap_example_on_demand() {
        let example = HashMapExample::new();
        assert_eq!(-5, example.get("5"));
        assert_eq!(-5, example.get("5"));
    }
}
