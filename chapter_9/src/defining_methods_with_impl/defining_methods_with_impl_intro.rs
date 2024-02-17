
pub struct Queue {
    older: Vec<char>, //older elements, eldest last.
    younger: Vec<char> // younger elements, younger last.
}

impl Queue {
    pub fn new() -> Queue {
        Queue { older: vec![], younger: vec![]}
    }

    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }
    
    /// Pop a character off the front of a queue. Return `Some(c)` if there
    /// was a character to pop, or `None` if the queue is empty.
    pub fn pop(&mut self) -> Option<char> {
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
}

impl Queue {
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl Queue {
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defining_methods_with_impl() {
        /// A first-in, first-out Queue of characters
        let mut q = Queue { older: Vec::new(), younger: Vec::new()};
        q.push('0');
        q.push('1');
        assert_eq!(q.pop(), Some('0'));

        q.push('∞');
        assert_eq!(q.pop(), Some('1'));
        assert_eq!(q.pop(), Some('∞'));
        assert_eq!(q.pop(), None);

        
        assert!(q.is_empty());
        q.push('☉');
        assert!(!q.is_empty());

        

        let mut q = Queue { older: Vec::new(), younger: Vec::new()};

        q.push('P');
        q.push('D');
        assert_eq!(q.pop(), Some('P'));
        q.push('X');

        let (older, younger) = q.split();
        // q is now uninitialized.
        assert_eq!(older, vec!['D']);
        assert_eq!(younger, vec!['X']);

    }
}