use crate::*;

pub struct PreOrderIter {
    stack: Vec<TreeIndex>
}

impl PreOrderIter {
    pub fn new(root: Option<TreeIndex>) -> Self {
        if let Some(index) = root {
            PreOrderIter {
                stack: vec![index]
            }
        } else {
            PreOrderIter {
                stack: vec![]
            }
        }
    }

    pub fn next(&mut self, tree: &Tree) -> Option<TreeIndex> {
        while let Some(node_index) = self.stack.pop() {
            if let Some(node) = tree.node_at(node_index) {
                if let Some(right) = node.right {
                    self.stack.push(right)
                }

                if let Some(left) = node.left {
                    self.stack.push(left)
                }

                return Some(node_index)
            }
        }

        None
    }
}
