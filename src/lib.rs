// A lot of this code is from this blog post:
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/

mod preorderiter;
use crate::preorderiter::*;

pub type TreeIndex = usize;

pub struct TreeNode<T: Sized + Copy> {
    pub value: T,
    pub left: Option<TreeIndex>,
    pub right: Option<TreeIndex>,
}

impl<T: Sized + Copy> TreeNode<T> {
    pub fn new(value: T, left: Option<TreeIndex>, right: Option<TreeIndex>) -> Self {
        TreeNode { value, left, right }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn node_height(&self, tree: &Tree<T>) -> usize {
        if self.left.is_none() && self.right.is_none() {
            return 0;
        }

        let mut left_height = 0;
        if let Some(left_index) = self.left {
            if let Some(left_node) = tree.node_at(left_index) {
                left_height = left_node.node_height(tree);
            }
        }
        let mut right_height = 0;
        if let Some(right_index) = self.right {
            if let Some(right_node) = tree.node_at(right_index) {
                right_height = right_node.node_height(tree);
            }
        }

        left_height.max(right_height) + 1
    }
}

pub struct Tree<T: Sized + Copy> {
    arena: Vec<Option<TreeNode<T>>>,
    root: Option<TreeIndex>,
}

impl<T: Sized + Copy> Tree<T> {
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn iter(&self) -> PreOrderIter {
        PreOrderIter::new(self.root)
    }

    pub fn set_root(&mut self, root: Option<TreeIndex>) {
        self.root = root;
    }

    pub fn add_node(&mut self, node: TreeNode<T>) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        index
    }

    pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode<T>> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode<T>> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        };
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode<T>> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        };
    }

    pub fn height(&self) -> Option<usize> {
        if let Some(root_index) = self.root {
            if let Some(root_node) = self.node_at(root_index) {
                return Some(root_node.node_height(self));
            }
        }
        None
    }

    pub fn tree_width(&self) -> Option<usize> {
        if let Some(root_index) = self.root {
            let mut index_stack = vec![root_index];
            let mut width = 0;

            while let Some(node_index) = index_stack.pop() {
                if let Some(node) = self.node_at(node_index) {
                    if let Some(left) = node.left {
                        index_stack.push(left);
                    }
                    if let Some(right) = node.right {
                        index_stack.push(right);
                    }
                    if width < index_stack.len() {
                        width = index_stack.len();
                    }
                }
            }
            return Some(width);
        }
        None
    }
}

impl<T: Sized + Copy> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////////////////////////
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = Tree::new();

        let a = tree.add_node(TreeNode::new(4, None, None));
        let b = tree.add_node(TreeNode::new(5, None, None));
        let c = tree.add_node(TreeNode::new(2, Some(a), Some(b)));

        let d = tree.add_node(TreeNode::new(3, None, None));
        let e = tree.add_node(TreeNode::new(1, Some(c), Some(d)));

        tree.set_root(Some(e));

        let mut preorder = tree.iter();
        while let Some(i) = preorder.next(&tree) {
            let node = tree.node_at_mut(i).expect("node to exist at given index");
            node.value += 10;
        }

        let mut preorder = tree.iter();
        while let Some(i) = preorder.next(&tree) {
            let node = tree.node_at(i).expect("node to exist at given index");
            println!("{}", node.value)
        }
    }

    #[test]
    fn node_height_simple() {
        let mut tree = Tree::new();

        let a = tree.add_node(TreeNode::new(4, None, None));
        let b = tree.add_node(TreeNode::new(5, None, None));
        let c = tree.add_node(TreeNode::new(2, Some(a), Some(b)));

        let d = tree.add_node(TreeNode::new(3, None, None));
        let e = tree.add_node(TreeNode::new(1, Some(c), Some(d)));

        tree.set_root(Some(e));
    }
}
