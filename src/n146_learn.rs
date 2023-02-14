#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{borrow::Borrow, cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

#[derive(Clone)]
struct KeyValue {
    value: i32,
    key: i32,
}

struct Node {
    value: KeyValue,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    length: usize,
}

struct LRUCache {
    capacity: usize,
    list: List,
    map: HashMap<i32, Rc<RefCell<Node>>>,
}

impl List {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn push_front(&mut self, value: KeyValue) -> Rc<RefCell<Node>> {
        self.length += 1;

        let new_node = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            value,
        }));

        // 直接take断掉指针
        let head = self.head.take();

        // 新头->旧头
        new_node.borrow_mut().next = head.clone();

        head.clone()
            .map(|x| x.borrow_mut().prev = Some(new_node.clone()));

        if self.length == 1 {
            self.tail = Some(new_node.clone());
        }

        self.head = Some(new_node.clone());
        new_node
    }

    fn remove(&mut self, target_node: Rc<RefCell<Node>>) {
        if self.length == 0 {
            return;
        }

        self.length -= 1;

        let prev = target_node.borrow_mut().prev.take();
        let next = target_node.borrow_mut().next.take();

        if prev.is_none() {
            self.head = next.clone();
        }
        if next.is_none() {
            self.tail = prev.clone();
        }

        prev.clone().map(|x| x.borrow_mut().next = next.clone());
        next.clone().map(|x| x.borrow_mut().prev = prev.clone());
    }

    fn pop(&mut self) -> Option<i32> {
        match self.tail.take() {
            Some(node) => {
                self.length -= 1;

                let prev = node.deref().borrow().prev.clone();
                prev.clone().map(|x| x.borrow_mut().next = None);
                node.borrow_mut().prev = None;

                self.tail = prev.clone();

                if self.length == 0 {
                    self.head = None;
                }

                Some(node.deref().borrow().value.key)
            }
            None => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // 回收所有节点
        while let Some(_) = self.pop() {}
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            list: List::new(),
            map: Default::default(),
        }
    }

    fn _get(&mut self, key: i32) -> Option<Rc<RefCell<Node>>> {
        match self.map.get_mut(&key) {
            Some(node) => {
                self.list.remove(node.clone());
                let new_node = self.list.push_front(node.borrow_mut().value.clone());
                *node = new_node.clone();
                Some(new_node)
            }
            None => None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self._get(key) {
            Some(node) => node.deref().borrow().value.value,
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self._get(key) {
            Some(node) => {
                node.borrow_mut().value.value = value;
            }
            None => {
                if self.list.length == self.capacity {
                    if let Some(remove_key) = self.list.pop() {
                        self.map.remove(&remove_key);
                    }
                }

                let node = self.list.push_front(KeyValue { value, key });
                self.map.insert(key, node);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut lRUCache = LRUCache::new(2);
        lRUCache.put(1, 1); // 缓存是 {1=1}
        lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
        lRUCache.get(1); // 返回 1
        lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
        lRUCache.get(2); // 返回 -1 (未找到)
        lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
        lRUCache.get(1); // 返回 -1 (未找到)
        lRUCache.get(3); // 返回 3
        lRUCache.get(4); // 返回 4
    }
}
