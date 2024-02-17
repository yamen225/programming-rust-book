// Simplified XML
use std::rc::Rc;

struct Node {
    tag: String,
    children: Vec<Rc<Node>>
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }
}

impl Node {
    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

#[cfg(test)]
mod tests {
    use super::super::defining_methods_with_impl_intro::Queue;
    use super::*;

    #[test]
    fn passing_self_as_a_box_rc_or_arc() { 
        let mut bq = Box::new(Queue::new());
        // `Queue::push` expects a `&mut Queue`, but `bq` is a `Box<Queue>`.
        // This is fine: Rust borrows a `&mut Queue` from the `Box` for the 
        // duration of the call
        bq.push('■');
        let parent = &mut Node::new("Parent");
        let shared_node = Rc::new(Node::new("first"));
        shared_node.append_to(parent)

    }
}