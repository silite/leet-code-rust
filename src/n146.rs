#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct LRUCache {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {}
    }

    fn get(&self, key: i32) -> i32 {
        -1
    }

    fn put(&self, key: i32, value: i32) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let obj = LRUCache::new(100);
        let ret_1: i32 = obj.get(10);
        obj.put(10, 10);
    }
}
