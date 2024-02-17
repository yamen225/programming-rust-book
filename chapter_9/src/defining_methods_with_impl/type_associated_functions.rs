pub struct Queue {
    older: Vec<char>, //older elements, eldest last.
    younger: Vec<char> // younger elements, younger last.
}

impl Queue {
    pub fn new() -> Queue {
        Queue {older: Vec::new(), younger: Vec::new()}

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_associated_functions() {
        let mut q = Queue::new();
    }
}