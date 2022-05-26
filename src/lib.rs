use std::collections::VecDeque;

#[derive(Debug)]
pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn count_recursive(&self) -> i32 {
        let count_left = self.left.as_ref().map_or(0, |node| node.count_recursive());
        let count_right = self.right.as_ref().map_or(0, |node| node.count_recursive());
        return 1 + count_left + count_right;
    }
}

#[derive(Debug)]
pub struct BinaryTree {
    root: Option<TreeNode>,
}

impl BinaryTree {
    pub fn new(root: TreeNode) -> Self {
        Self { root: Some(root) }
    }

    pub fn new_empty() -> Self {
        Self { root: None }
    }

    pub fn count_recursive(&self) -> i32 {
        self.root.as_ref().map_or(0, |node| node.count_recursive())
    }

    pub fn count_iterative(&self) -> i32 {
        let mut count = 0;
        let mut queue = VecDeque::<&TreeNode>::new();
        if let Some(ref node) = self.root {
            queue.push_back(node);
        };
        while let Some(front) = queue.pop_front() {
            count += 1;
            if let Some(left) = front.left.as_deref() {
                queue.push_back(left);
            }
            if let Some(right) = front.right.as_deref() {
                queue.push_back(right);
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // (empty)
    #[test]
    fn count_empty_tree() {
        let tree = BinaryTree::new_empty();
        assert_eq!(tree.count_recursive(), 0);
        assert_eq!(tree.count_iterative(), 0);
    }

    //   1
    //  /
    // 2
    #[test]
    fn count_tree_1() {
        let root = TreeNode {
            value: 1,
            left: Some(Box::new(TreeNode {
                value: 2,
                left: None,
                right: None,
            })),
            right: None,
        };
        let tree = BinaryTree::new(root);
        assert_eq!(tree.count_recursive(), 2);
        assert_eq!(tree.count_iterative(), 2);
    }

    //   1
    //    \
    //     3
    #[test]
    fn count_tree_2() {
        let root = TreeNode {
            value: 1,
            left: None,
            right: Some(Box::new(TreeNode {
                value: 3,
                left: None,
                right: None,
            })),
        };
        let tree = BinaryTree::new(root);
        assert_eq!(tree.count_recursive(), 2);
        assert_eq!(tree.count_iterative(), 2);
    }

    //          1
    //         /
    //        2
    //       /
    //      3
    //     /
    //   ...
    //   /
    //  n
    #[test]
    fn count_tree_3() {
        let n = 50;
        let mut root = TreeNode {
            value: 1,
            left: None,
            right: None,
        };
        let mut current = &mut root;
        for i in 2..=n {
            current.left = Some(Box::new(TreeNode {
                value: i,
                left: None,
                right: None,
            }));
            current = current.left.as_deref_mut().unwrap();
        }
        let tree = BinaryTree::new(root);
        assert_eq!(tree.count_recursive(), n);
        assert_eq!(tree.count_iterative(), n);
    }

    //          1
    //      /      \
    //     2        3
    //    / \      / \
    //   4   5    6   7
    #[test]
    fn count_tree_4() {
        let root = TreeNode {
            value: 1,
            left: Some(Box::new(TreeNode {
                value: 2,
                left: Some(Box::new(TreeNode {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 5,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                value: 3,
                left: Some(Box::new(TreeNode {
                    value: 6,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 7,
                    left: None,
                    right: None,
                })),
            })),
        };
        let tree = BinaryTree::new(root);
        assert_eq!(tree.count_recursive(), 7);
        assert_eq!(tree.count_iterative(), 7);
    }
}
