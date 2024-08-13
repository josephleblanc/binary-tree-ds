// A lot of this code is from this blog post:
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/

mod preorderiter;
use crate::preorderiter::*;

pub type TreeIndex = usize;

pub struct TreeNode {
    pub value: usize,
    pub left: Option<TreeIndex>,
    pub right: Option<TreeIndex>,
}

impl TreeNode {
    pub fn new(
        value: usize,
        left: Option<TreeIndex>,
        right: Option<TreeIndex>
    ) -> Self {
        TreeNode {
            value,
            left,
            right
        }
    }
}

pub struct Tree {
    arena: Vec<Option<TreeNode>>,
    root: Option<TreeIndex>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            arena: Vec::new(),
            root: None
        }
    }

    pub fn iter(&self) -> PreOrderIter {
        PreOrderIter::new(self.root)
    }

    pub fn set_root(&mut self, root: Option<TreeIndex>) {
        self.root = root;
    }

    pub fn add_node(&mut self, node: TreeNode) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        index
    }

    pub fn remove_node_at(&mut self, index: TreeIndex) -> Option<TreeNode> {
        if let Some(node) = self.arena.get_mut(index) {
            node.take()
        } else {
            None
        }
    }

    pub fn node_at(&self, index: TreeIndex) -> Option<&TreeNode> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        }
    }

    pub fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut TreeNode> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        }
    }
}

impl Default for Tree {
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
}
