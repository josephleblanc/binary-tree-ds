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
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
