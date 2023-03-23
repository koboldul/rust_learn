pub struct Queue<T> {
    old_data: Vec<T>,
    new_data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            old_data: Vec::new(),
            new_data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.new_data.push(val);
    }

    pub fn is_empty(&self) -> bool {
        self.old_data.is_empty() && self.new_data.is_empty()
    }
}