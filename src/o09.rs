//  用两个栈实现队列 https://leetcode.cn/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/?envType=study-plan&id=lcof&plan=lcof&plan_progress=b61si4t

pub struct CQueue {
    value: Vec<i32>,
    head: i32,
    tail: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    pub fn new() -> Self {
        CQueue {
            value: Vec::with_capacity(10000),
            head: 0,
            tail: 0,
        }
    }

    pub fn append_tail(&mut self, value: i32) {
        self.value.push(value);
        self.tail += 1;
    }

    pub fn delete_head(&mut self) -> i32 {
        if self.head >= self.tail {
            -1
        } else {
            self.head += 1;
            self.value[(self.head - 1) as usize]
        }
    }
}
