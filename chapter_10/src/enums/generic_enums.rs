enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Error(E)
}

/// An ordered collection of `T`s.
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

/// A part of a BinaryTree.
struct TreeNode <T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

#[cfg(test)]
mod tests{
    use std::io::Empty;

    use super::{BinaryTree, TreeNode};

    fn test_binary_tree() {
        let jupiter_tree = BinaryTree::NonEmpty(
            Box::new(
                TreeNode {
                    element: "Jupiter", 
                    left: BinaryTree::Empty, 
                    right: BinaryTree::Empty 
                }
            )
        );

        let mercury_tree = BinaryTree::NonEmpty(
            Box::new(
                TreeNode { element: "Mercury", left: BinaryTree::Empty, right: BinaryTree::Empty }
            )
        );

        let mars_tree = BinaryTree::NonEmpty(
            Box::new(
                TreeNode {
                    element: "Mars",
                    left: jupiter_tree,
                    right: mercury_tree
                }
            )
        );
    }
}