// A lot of this code is from this blog post:
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/

use std::borrow::BorrowMut;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
//use std::error::Error;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

mod preorderiter;
//use crate::preorderiter::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Tree<T: Sized + Copy> {
    pub root: TreeNodeRef<T>,
}

//pub struct TypstConfig {
//
//}

impl<T: Sized + Copy + Debug + Display> Tree<T> {
    pub fn new(root: TreeNodeRef<T>) -> Self {
        Tree { root }
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<TreeNodeRef<T>> {
        self.root.get_by_id(id)
    }

    /// Get the `Rc<RefCell>` of the parent of the node passed as argument.
    ///
    // e.g. To get calling tree.get_parent(&node_ref4) on the following tree returns node_ref2:
    //
    //     1
    //    / \
    //   2   3
    //  / \
    // 4   5
    //
    pub fn get_parent(&self, node_ref: &TreeNodeRef<T>) -> TreeNodeRef<T> {
        let node = node_ref.borrow();
        // TODO: error handling
        Tree::get_parent_rec(&self.root, node.id).expect("Node not found")
    }

    fn get_parent_rec(node_ref: &TreeNodeRef<T>, find_id: Uuid) -> Option<TreeNodeRef<T>> {
        let node = node_ref.borrow();
        let debug_value = node.value;
        println!("debug_value: {}", debug_value);

        if let Some(ref right) = node.right {
            if right.borrow().id == find_id {
                // Clones the Rc, not the value
                return Some(node_ref.clone());
            }
            let right_opt = Tree::get_parent_rec(right, find_id);
            if right_opt.is_some() {
                return right_opt;
            }
        }
        if let Some(ref left) = node.left {
            if left.borrow().id == find_id {
                // Clones the Rc, not the value
                return Some(node_ref.clone());
            }
            let left_opt = Tree::get_parent_rec(left, find_id);
            if left_opt.is_some() {
                return left_opt;
            }
        }
        None
    }

    pub fn max_depth(&self) -> isize {
        let root: &TreeNode<T> = &self.root.borrow();
        let node_vec = root.pre_order_vec();

        let mut depth: isize = TreeNode::depth(root, &node_vec[0].borrow().id);
        for node in node_vec {
            let node_depth = TreeNode::depth(root, &node.borrow().id);
            if depth < node_depth {
                depth = node_depth;
            }
        }
        depth
    }

    /// The maximum possible width of the tree, given the depth. This is not the same as the max
    /// width. Where max width is the greatest number of nodes on a level, the max possible width
    /// is the max width if all nodes up to the tree's depth were populated.
    pub fn max_width_upper(&self) -> isize {
        let depth: isize = self.max_depth();

        // max width is properly 1 on a lone leaf.
        if depth == 0 {
            return 1;
        }
        1 + 2_isize.pow(depth.try_into().unwrap())
    }

    /// Takes the tree and formats it into a typst representation of a binary tree node structure.
    /// The format of the tree node was taken from:
    /// https://sitandr.github.io/typst-examples-book/book/packages/graphs.html
    pub fn typst_string(&self) -> String {
        let mut out_string = String::new();

        let root = self.root.borrow();
        out_string.push_str(root.format_typst().as_str());

        out_string
    }

    pub fn save_typst(&self, file: &'static str) -> std::io::Result<()> {
        let mut f = File::create(file)?;
        let pre_string = r#"
#let data = (
"#;
        // Formatted tree nodes go here,
        // e.g.
        //[A], ([B], [C], [D]), ([E], [F])

        let post_string = r#"
)

#import "@preview/cetz:0.1.2": canvas, draw, tree

#canvas(length: 1cm, {
  import draw: *

  set-style(content: (padding: .2),
    fill: gray.lighten(70%),
    stroke: gray.lighten(70%))

  tree.tree(data, spread: 2.5, grow: 1.5, draw-node: (node, _) => {
    circle((), radius: .45, stroke: none)
    content((), node.content)
  }, draw-edge: (from, to, _) => {
    line((a: from, number: .6, abs: true, b: to),
         (a: to, number: .6, abs: true, b: from), mark: (end: ">"))
  }, name: "tree")
})
"#;
        let mut out_string = String::from(pre_string);

        out_string.push_str(self.typst_string().as_str());
        out_string.push_str(post_string);

        f.write_all(out_string.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode<T: Sized + Copy> {
    pub value: T,
    pub left: Option<TreeNodeRef<T>>,
    pub right: Option<TreeNodeRef<T>>,
    pub(crate) id: Uuid,
}

/// Tests for equality between contents of trees. To check if the trees are identical use:
///
/// node1 == node2 && node1.id == node2.id
///
/// The differentiation is necessary for the uuid created upon creation of the different trees,
/// which is used when identifying that specific node in an internal function (namely the .depth()
/// function)
impl<T: Sized + Copy + PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}

type TreeNodeRef<T: Sized + Copy> = Rc<RefCell<TreeNode<T>>>;

impl<T: Sized + Copy + Display> TreeNode<T> {
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    pub fn format_typst(&self) -> String {
        if self.is_leaf() {
            return format!("[{}]", self.value);
        }
        let mut out = String::from("(");

        out.push_str(format!("[{}], ", self.value).as_str());
        if let Some(left) = self.left.clone() {
            out.push_str(format!("{}, ", left.borrow().format_typst()).as_str());
        }
        if let Some(right) = self.right.clone() {
            out.push_str(right.borrow().format_typst().as_str());
        }
        out.push(')');

        out
    }

    pub fn cmp_id(&self, id: Uuid) -> bool {
        self.id == id
    }

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
        self.left.is_none() && self.right.is_none()
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

    pub fn pre_order_vec(&self) -> Vec<TreeNodeRef<T>> {
        // Not sure about possible repurcussions from the clone below, if both contain Rc's to the
        // same nodes.
        // TODO: Add some tests for what happens if the nodes are changed through the vector and/or
        // through the original tree.
        let start_node: TreeNodeRef<T> = Rc::new(RefCell::new(self.clone()));
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
    pub fn depth(root: &TreeNode<T>, _id: &Uuid) -> isize {
        let mut dist: isize = -1;

        // Check if current node is target node;
        if &root.id == _id {
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

pub trait TreeNodeProperties<T: Copy + Sized> {
    fn get_by_id(&self, id: Uuid) -> Option<TreeNodeRef<T>>;
}

//impl<T: Copy + Sized + Display> TreeNodeProperties<T> for TreeNodeRef<T> {
//    fn get_by_id(&self, id: Uuid) -> Option<TreeNodeRef<T>> {
//        if self.borrow().id == id {
//            return Some(self.clone());
//        }
//
//        let mut right_ret: Option<TreeNodeRef<T>> = None;
//        let mut left_ret: Option<TreeNodeRef<T>> = None;
//        if let Some(ref right) = self.borrow().right {
//            right_ret = right.get_by_id(id);
//        }
//        if let Some(ref left) = self.borrow().left {
//            left_ret = left.get_by_id(id);
//        }
//
//        if right_ret.is_some() {
//            return right_ret;
//        }
//        left_ret
//    }
//}

/// Search children for node by uuid.
impl<T: Copy + Sized + Display> TreeNodeProperties<T> for TreeNodeRef<T> {
    fn get_by_id(&self, id: Uuid) -> Option<TreeNodeRef<T>> {
        if self.borrow().id == id {
            return Some(self.clone());
        }

        if let Some(ref right) = self.borrow().right {
            let right_ret = right.get_by_id(id);
            if right_ret.is_some() {
                return right_ret;
            }
        }
        if let Some(ref left) = self.borrow().left {
            let left_ret = left.get_by_id(id);
            if left_ret.is_some() {
                return left_ret;
            }
        }

        None
    }
}

////////////////////////////////////////////////////////////////////////////////
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

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

        let depth1 = TreeNode::depth(&node1, &node1.id);
        let depth2 = TreeNode::depth(&node1, &node2.borrow().id);
        let depth3 = TreeNode::depth(&node1, &node3.borrow().id);
        let depth4 = TreeNode::depth(&node1, &node4.borrow().id);
        let depth5 = TreeNode::depth(&node1, &node5.borrow().id);
        let depth6 = TreeNode::depth(&node1, &node6.borrow().id);
        let depth7 = TreeNode::depth(&node1, &node7.borrow().id);
        let depth8 = TreeNode::depth(&node1, &node8.borrow().id);

        assert_eq!(0, depth1);
        assert_eq!(1, depth2);
        assert_eq!(1, depth3);
        assert_eq!(2, depth4);
        assert_eq!(2, depth5);
        assert_eq!(2, depth6);
        assert_eq!(3, depth7);
        assert_eq!(3, depth8);
    }

    #[test]
    fn tree_depth() {
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

        let node1_rc = Rc::new(RefCell::new(node1));

        let tree = Tree::new(node1_rc.clone());
        let node_id = node7.clone().borrow().id;
        assert_eq!(
            tree.max_depth(),
            TreeNode::depth(&node1_rc.borrow(), &node_id)
        );
        assert_eq!(tree.max_depth(), 3);
    }
    #[test]
    fn tree_max_width_possible() {
        // Test node depth on the following tree:
        //                 1
        //                / \
        //               2   3
        //              / \   \
        //             4   5   6
        //                /     \
        //               7       8
        //
        // max possible width: 1 + 2^3 = 9
        let node4 = TreeNode::new_rc(4, None, None);
        let node7 = TreeNode::new_rc(7, None, None);
        let node8 = TreeNode::new_rc(7, None, None);

        let node6 = TreeNode::new_rc(6, None, Some(node8.clone()));
        let node3 = TreeNode::new_rc(3, None, Some(node6.clone()));
        let node5 = TreeNode::new_rc(5, Some(node7.clone()), None);

        let node2 = TreeNode::new_rc(2, Some(node4.clone()), Some(node5.clone()));

        let node1 = TreeNode::new(1, Some(node2.clone()), Some(node3.clone()));

        let node1_rc = Rc::new(RefCell::new(node1));

        let tree = Tree::new(node1_rc);
        assert_eq!(tree.max_width_upper(), 9);

        let tree = Tree::new(node2);
        assert_eq!(tree.max_width_upper(), 5);

        let tree = Tree::new(node5);
        assert_eq!(tree.max_width_upper(), 3);

        let tree = Tree::new(node7);
        assert_eq!(tree.max_width_upper(), 1);
    }

    #[test]
    fn test_create_typst_string() {
        // Test node depth on the following tree:
        //                 1
        //                / \
        //               2   3
        //              / \
        //             4   5
        //
        // max possible width: 1 + 2^3 = 9
        let node4 = TreeNode::new_rc(4, None, None);

        let node3 = TreeNode::new_rc(3, None, None);
        let node5 = TreeNode::new_rc(5, None, None);

        let node2 = TreeNode::new_rc(2, Some(node4.clone()), Some(node5.clone()));

        let node1 = TreeNode::new(1, Some(node2.clone()), Some(node3.clone()));

        let node1_rc = Rc::new(RefCell::new(node1));

        //let tree = Tree::new(node1_rc);
        let tree5_node = Tree::new(node5);
        assert_eq!(tree5_node.typst_string(), "[5]");
        assert_ne!(tree5_node.typst_string(), "[1]");

        let tree2 = Tree::new(node2);
        assert_eq!(tree2.typst_string(), "([2], [4], [5])");

        let tree1 = Tree::new(node1_rc);
        assert_eq!(tree1.typst_string(), "([1], ([2], [4], [5]), [3])");
    }

    #[test]
    fn save_typst() {
        // Test node depth on the following tree:
        //                 1
        //                / \
        //               2   3
        //              / \
        //             4   5
        //
        // max possible width: 1 + 2^3 = 9
        let node4 = TreeNode::new_rc(4, None, None);

        let node3 = TreeNode::new_rc(3, None, None);
        let node5 = TreeNode::new_rc(5, None, None);

        let node2 = TreeNode::new_rc(2, Some(node4.clone()), Some(node5.clone()));

        let node1 = TreeNode::new(1, Some(node2.clone()), Some(node3.clone()));

        let node1_rc = Rc::new(RefCell::new(node1));
        let tree1 = Tree::new(node1_rc);
        tree1.save_typst("./typst_test.typ");

        let mut file = File::open("./typst_test.typ").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(
            contents,
            r#"
#let data = (
([1], ([2], [4], [5]), [3])
)

#import "@preview/cetz:0.1.2": canvas, draw, tree

#canvas(length: 1cm, {
  import draw: *

  set-style(content: (padding: .2),
    fill: gray.lighten(70%),
    stroke: gray.lighten(70%))

  tree.tree(data, spread: 2.5, grow: 1.5, draw-node: (node, _) => {
    circle((), radius: .45, stroke: none)
    content((), node.content)
  }, draw-edge: (from, to, _) => {
    line((a: from, number: .6, abs: true, b: to),
         (a: to, number: .6, abs: true, b: from), mark: (end: ">"))
  }, name: "tree")
})
"#
        );
    }
    #[test]
    fn get_parent() {
        // Test tree:
        //                 1
        //                / \
        //               2   3
        //              / \
        //             4   5
        //
        let node4 = TreeNode::new_rc(4, None, None);

        let node3 = TreeNode::new_rc(3, None, None);
        let node5 = TreeNode::new_rc(5, None, None);

        let node2 = TreeNode::new_rc(2, Some(node4.clone()), Some(node5.clone()));

        let node1 = TreeNode::new(1, Some(node2.clone()), Some(node3.clone()));

        let node1_rc = Rc::new(RefCell::new(node1));
        let tree1 = Tree::new(node1_rc);

        let node4_parent = tree1.get_parent(&node4);

        for node in tree1.root.borrow().pre_order_vec() {
            println!(
                "node val: {}, node id: {}",
                node.borrow().value,
                node.borrow().id
            );
        }
        assert_eq!(node2.borrow().id, node4_parent.borrow().id);
    }

    #[test]
    fn test_get_by_id() {
        // Test tree:
        //                 1
        //                / \
        //               2   3
        //              / \
        //             4   5
        //
        let node4 = TreeNode::new_rc(4, None, None);

        let node3 = TreeNode::new_rc(3, None, None);
        let node5 = TreeNode::new_rc(5, None, None);

        let node2 = TreeNode::new_rc(2, Some(node4.clone()), Some(node5.clone()));

        let node1 = TreeNode::new(1, Some(node2.clone()), Some(node3.clone()));

        let node1_rc = Rc::new(RefCell::new(node1.clone()));
        let tree = Tree::new(node1_rc.clone());

        println!(
            "node1_rc val: {}, node id: {}",
            node1_rc.borrow().value,
            node1_rc.borrow().id,
        );

        for node in tree.root.borrow().pre_order_vec() {
            println!(
                "node val: {}, node id: {}",
                node.borrow().value,
                node.borrow().id
            );
        }
        assert_eq!(node3, tree.get_by_id(node3.borrow().id).unwrap());
        assert_ne!(node2, tree.get_by_id(node3.borrow().id).unwrap());
        assert_eq!(node1_rc, tree.get_by_id(node1.id).unwrap());
        assert_eq!(node5, tree.get_by_id(node5.borrow().id).unwrap());
    }
}
