fn main() {
    println!("Hello, world!");
}

struct MinStack {
    stack: Vec<i32>,
    top_id: usize,
    min_val: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: vec![i32::MAX],
            top_id: 0,
            min_val: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        self.top_id += 1;
        self.stack.push(val);
        if val < self.min_val {
            self.min_val = val;
        }
    }

    fn pop(&mut self) {
        self.top_id -= 1;
        let val = self.stack.pop().unwrap();
        if val == self.min_val {
            let stack = &self.stack;
            self.min_val = *stack.into_iter().min_by(|a, b| a.cmp(&b)).unwrap();
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.top_id]
    }

    fn get_min(&self) -> i32 {
        self.min_val
    }
}
