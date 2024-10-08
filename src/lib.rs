// A lot of this code is from this blog post:
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/

use std::borrow::BorrowMut;
use std::error::Error;
use std::{cell::RefCell, rc::Rc};

mod preorderiter;
//use crate::preorderiter::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T: Sized + Copy> {
    pub value: T,
    pub left: Option<TreeNodeRef<T>>,
    pub right: Option<TreeNodeRef<T>>,
}

type TreeNodeRef<T: Sized + Copy> = Rc<RefCell<TreeNode<T>>>;

impl<T: Sized + Copy> TreeNode<T> {
    pub fn new(value: T, left: Option<TreeNodeRef<T>>, right: Option<TreeNodeRef<T>>) -> Self {
        TreeNode { value, left, right }
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
}

//pub struct Tree<T: Sized + Copy> {
//    root: Option<TreeNodeRef<T>>,
//}

//impl<T: Sized + Copy> Tree<T> {
//pub fn new() -> Self {
//    Tree {
//        arena: Vec::new(),
//        root: None,
//    }
//}
//
//pub fn iter(&self) -> PreOrderIter {
//    PreOrderIter::new(self.root)
//}
//
//pub fn set_root(&mut self, root: Option<TreeIndex>) {
//    self.root = root;
//}
//
//pub fn add_node(&mut self, node: TreeNode<T>) -> TreeIndex {
//    let index = self.arena.len();
//    self.arena.push(Some(node));
//    index
//}
//
//pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode<T>> {
//    if let Some(node) = self.arena.get_mut(index) {
//        node.take()
//    } else {
//        None
//    }
//}
//
//pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode<T>> {
//    return if let Some(node) = self.arena.get(index) {
//        node.as_ref()
//    } else {
//        None
//    };
//}
//
//pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode<T>> {
//    return if let Some(node) = self.arena.get_mut(index) {
//        node.as_mut()
//    } else {
//        None
//    };
//}
//
//pub fn height(&self) -> Option<usize> {
//    if let Some(root_index) = self.root {
//        if let Some(root_node) = self.node_at(root_index) {
//            return Some(root_node.node_height(self));
//        }
//    }
//    None
//}
//
//pub fn tree_width(&self) -> Option<usize> {
//    if let Some(root_index) = self.root {
//        let mut index_stack = vec![root_index];
//        let mut width = 0;
//
//        while let Some(node_index) = index_stack.pop() {
//            if let Some(node) = self.node_at(node_index) {
//                if let Some(left) = node.left {
//                    index_stack.push(left);
//                }
//                if let Some(right) = node.right {
//                    index_stack.push(right);
//                }
//                if width < index_stack.len() {
//                    width = index_stack.len();
//                }
//            }
//        }
//        return Some(width);
//    }
//    None
//}
//}

//impl<T: Sized + Copy> Default for Tree<T> {
//    fn default() -> Self {
//        Self::new()
//    }
//}

////////////////////////////////////////////////////////////////////////////////
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let mut tree = Tree::new();
        //
        //let a = tree.add_node(TreeNode::new(4, None, None));
        //let b = tree.add_node(TreeNode::new(5, None, None));
        //let c = tree.add_node(TreeNode::new(2, Some(a), Some(b)));
        //
        //let d = tree.add_node(TreeNode::new(3, None, None));
        //let e = tree.add_node(TreeNode::new(1, Some(c), Some(d)));
        //
        //tree.set_root(Some(e));
        //
        //let mut preorder = tree.iter();
        //while let Some(i) = preorder.next(&tree) {
        //    let node = tree.node_at_mut(i).expect("node to exist at given index");
        //    node.value += 10;
        //}
        //
        //let mut preorder = tree.iter();
        //while let Some(i) = preorder.next(&tree) {
        //    let node = tree.node_at(i).expect("node to exist at given index");
        //    println!("{}", node.value)
        //}
    }

    #[test]
    fn node_height_simple() {
        //let mut tree = Tree::new();
        //
        //let a = tree.add_node(TreeNode::new(4, None, None));
        //let b = tree.add_node(TreeNode::new(5, None, None));
        //let c = tree.add_node(TreeNode::new(2, Some(a), Some(b)));
        //
        //let d = tree.add_node(TreeNode::new(3, None, None));
        //let e = tree.add_node(TreeNode::new(1, Some(c), Some(d)));
        //
        //tree.set_root(Some(e));
    }
}
