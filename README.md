# Simple Binary Tree

Define binary tree as a single root TreeNode. A TreeNode contains a value, and optionally left TreeNode and right TreeNode.

```rust
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct BinaryTree<T> {
    root: Option<TreeNode<T>>,
}
```

To count the number of nodes, call method `BinaryTree::count_recursive` or `BinaryTree::count_iterative` on a `BinaryTree` instance.

The methods are demonstrated by test cases written in `src/lib.rs`. To run the test cases:

```
cargo test
```
