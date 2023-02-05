struct MinStack {
    min: Vec<i32>,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            min: Vec::new(),
            stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min.is_empty() {
            self.min.push(x);
        } else {
            let last = *self.min.last().unwrap();
            self.min.push(last.min(x));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let i = 3;
        let res = i.min(2);
        println!("{}", res);
    }
}
