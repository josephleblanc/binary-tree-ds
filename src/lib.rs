// A lot of this code is from this blog post:
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/

use std::borrow::BorrowMut;
use std::error::Error;
use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

mod preorderiter;
//use crate::preorderiter::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T: Sized + Copy> {
    pub value: T,
    pub left: Option<TreeNodeRef<T>>,
    pub right: Option<TreeNodeRef<T>>,
    pub(crate) id: Uuid,
}

type TreeNodeRef<T: Sized + Copy> = Rc<RefCell<TreeNode<T>>>;

impl<T: Sized + Copy> TreeNode<T> {
    pub fn new(value: T, left: Option<TreeNodeRef<T>>, right: Option<TreeNodeRef<T>>) -> Self {
        TreeNode {
            value,
            left,
            right,
            id: Uuid::new_v4(),
        }
    }

    pub fn new_rc(
        value: T,
        left: Option<TreeNodeRef<T>>,
        right: Option<TreeNodeRef<T>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left,
            right,
            id: Uuid::new_v4(),
        }))
    }

    pub fn add_left(&mut self, value: T) {
        self.left = Some(Rc::new(RefCell::new(TreeNode::new(value, None, None))));
    }

    pub fn add_right(&mut self, value: T) {
        self.right = Some(Rc::new(RefCell::new(TreeNode::new(value, None, None))));
    }

    pub fn set_left(&mut self, node: TreeNode<T>) {
        self.left = Some(Rc::new(RefCell::new(node)));
    }

    pub fn set_right(&mut self, node: TreeNode<T>) {
        self.right = Some(Rc::new(RefCell::new(node)));
    }

    pub fn add_leaf(&mut self, leaf: T) -> Result<(), String> {
        let node = self.borrow_mut();
        if node.left.is_none() {
            node.add_left(leaf);
            return Ok(());
        } else if node.right.is_none() {
            node.add_right(leaf);
            return Ok(());
        }
        Err("Attempted to add a leaf to a full node".to_string())
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.left.is_none()
    }

    pub fn count_children(&self) -> usize {
        let mut count = 0;
        if self.left.is_some() {
            count += 1;
        }
        if self.right.is_some() {
            count += 1;
        }

        count
    }

    pub fn count(self) -> usize {
        let mut node_total: usize = 0;
        let start_node: Rc<RefCell<TreeNode<T>>> = Rc::new(RefCell::new(self));
        let mut stack: Vec<TreeNodeRef<T>> = vec![start_node];

        //while !stack.is_empty() {
        while let Some(current) = stack.pop() {
            node_total += 1;

            //let current: Rc<RefCell<TreeNode<T>>> = stack.pop().unwrap();
            if let Some(left) = &current.borrow().left {
                stack.push(left.to_owned());
            };
            if let Some(right) = &current.borrow().right {
                stack.push(right.to_owned());
            };
        }
        node_total
    }

    pub fn pre_order_vec(self) -> Vec<TreeNodeRef<T>> {
        let start_node: TreeNodeRef<T> = Rc::new(RefCell::new(self));
        let mut traverse_stack: Vec<TreeNodeRef<T>> = vec![start_node.clone()];
        let mut pre_order_vec: Vec<TreeNodeRef<T>> = vec![start_node];

        while let Some(current) = traverse_stack.pop() {
            if let Some(left) = &current.borrow().left {
                pre_order_vec.push(left.clone());
                traverse_stack.push(left.to_owned());
            };
            if let Some(right) = &current.borrow().right {
                pre_order_vec.push(right.clone());
                traverse_stack.push(right.to_owned());
            };
        }
        pre_order_vec
    }

    /// Function to find the depth of a given node in a binary tree. The depth is the depth between
    /// the root node and the id in the second argument. For example, entering the root's own id
    /// returns 0, while the root's child will return 1, etc.
    /// An example of what a tree and depth from the root would look like below:
    /// depth: 0        1
    ///                / \
    /// depth: 1      2   3
    ///              / \   \
    /// depth: 2    4   5   6
    ///                /     \
    /// depth: 3      8       7
    ///
    // Algorithm taken from C++ implementation at:
    // https://www.geeksforgeeks.org/height-and-depth-of-a-node-in-a-binary-tree/
    pub fn depth(root: &TreeNode<T>, _id: Uuid) -> isize {
        let mut dist: isize = -1;

        // Check if current node is target node;
        if root.id == _id {
            return dist + 1;
        }
        if let Some(left) = &root.left {
            dist = TreeNode::depth(&left.borrow(), _id);
            if dist >= 0 {
                return dist + 1;
            }
        }
        if let Some(right) = &root.right {
            dist = TreeNode::depth(&right.borrow(), _id);
            if dist >= 0 {
                return dist + 1;
            }
        }

        dist
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
    fn it_works() {}

    #[test]
    fn node_depth() {
        // Test node depth on the following tree:
        // depth: 0        1
        //                / \
        // depth: 1      2   3
        //              / \   \
        // depth: 2    4   5   6
        //                /     \
        // depth: 3      7       8
        //
        let node4 = TreeNode::new_rc(4, None, None);
        let node7 = TreeNode::new_rc(7, None, None);
        let node8 = TreeNode::new_rc(7, None, None);

        let node6 = TreeNode::new_rc(6, None, Some(node8.clone()));
        let node3 = TreeNode::new_rc(3, None, Some(node6.clone()));
        let node5 = TreeNode::new_rc(5, Some(node7.clone()), None);

        let node2 = TreeNode::new_rc(2, Some(node4.clone()), Some(node5.clone()));

        let node1 = TreeNode::new(1, Some(node2.clone()), Some(node3.clone()));

        let depth1 = TreeNode::depth(&node1, node1.id);
        let depth2 = TreeNode::depth(&node1, node2.borrow().id);
        let depth3 = TreeNode::depth(&node1, node3.borrow().id);
        let depth4 = TreeNode::depth(&node1, node4.borrow().id);
        let depth5 = TreeNode::depth(&node1, node5.borrow().id);
        let depth6 = TreeNode::depth(&node1, node6.borrow().id);
        let depth7 = TreeNode::depth(&node1, node7.borrow().id);
        let depth8 = TreeNode::depth(&node1, node8.borrow().id);

        assert_eq!(0, depth1);
        assert_eq!(1, depth2);
        assert_eq!(1, depth3);
        assert_eq!(2, depth4);
        assert_eq!(2, depth5);
        assert_eq!(2, depth6);
        assert_eq!(3, depth7);
        assert_eq!(3, depth8);
    }
}
