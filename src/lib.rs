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
    id: Uuid,
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

    /// Function to find the depth of a given node in a binary tree.
    // Algorithm taken from C++ implementation at:
    // https://www.geeksforgeeks.org/height-and-depth-of-a-node-in-a-binary-tree/
    pub fn depth(&self) -> isize {
        // Base Case
        if self.is_leaf() {
            return 0;
        }
        todo!();
    }
}

////////////////////////////////////////////////////////////////////////////////
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn node_height_simple() {}
}
