// A lot of this code is from this blog post:
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/

pub struct TreeNode {
    pub value: usize,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(
        value: usize,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    ) -> Self {
        TreeNode {
            value,
            left,
            right
        }
    }
}

pub struct Tree {
    root: Option<TreeNode>
}

impl Tree {
    pub fn new(root: Option<TreeNode>) -> Self {
        Tree {
            root
        }
    }

    pub fn iter(&self) -> PreOrderIter {
        PreOrderIter::new(self.root.as_ref())
    }
}

pub struct PreOrderIter<'a> {
    stack: Vec<&'a TreeNode>
}

impl<'a> PreOrderIter<'a> {
    pub fn new(root: Option<&'a TreeNode>) -> Self {
        if let Some(node) = root {
            PreOrderIter {
                stack: vec![node]
            }
        } else {
            PreOrderIter {
                stack: vec![]
            }
        }
    }
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = &'a TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = &node.right {
                self.stack.push(right)
            }
        }

        if let Some(node) = self.stack.pop() {
            if let Some(left) = &node.left {
                self.stack.push(left)
            }
        }

    None
    }
}



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
