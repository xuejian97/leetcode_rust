struct MyCircularQueue {
    queue: Vec<i32>,
    front_idx: usize,
    rear_idx: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            queue: vec![-1; k as usize + 1],
            front_idx: 0,
            rear_idx: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() { return false; }
        self.queue[self.rear_idx] = value;
        if self.rear_idx == self.queue.len() - 1 {
            self.rear_idx = 0;
        } else {
            self.rear_idx += 1;
        }
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() { return false; }
        if self.front_idx == self.queue.len() - 1 {
            self.front_idx = 0;
        } else {
            self.front_idx += 1;
        }
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() { return -1; }
        self.queue[self.front_idx]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() { return -1; }
        if self.rear_idx == 0 {
            self.queue[self.queue.len() - 1]
        } else {
            self.queue[self.rear_idx - 1]
        }
    }

    fn is_empty(&self) -> bool {
        self.front_idx == self.rear_idx
    }

    fn is_full(&self) -> bool {
        use std::cmp::Ordering;

        match self.rear_idx.cmp(&self.front_idx) {
            Ordering::Less => {
                self.front_idx - 1 == self.rear_idx
            }
            Ordering::Greater => {
                self.front_idx == 0 && self.rear_idx == self.queue.len() - 1
            }
            Ordering::Equal => {
                false
            }
        }
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

fn main() {}