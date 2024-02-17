pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Queue<T>{
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t)
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty(){
                return None;
            }
            // Bring the elements in younger over to older, and put them in
            // the promised order
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_structs() {
        // let mut q = Queue::<char>::new();

        let mut q = Queue::new();
        let mut r = Queue::new();

        q.push("CAD"); // aparently a Queue<&'static str>
        r.push(0.74); // aparently a Queue<f64>

        q.push("BTC"); //Bitcoins per USD, 2019-6
        r.push(13764.0); //Rust fails to detect irrational exuberance
    }
}