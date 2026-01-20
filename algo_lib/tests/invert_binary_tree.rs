use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let left = Self::invert_tree(node.borrow().left.clone());
                let right = Self::invert_tree(node.borrow().right.clone());
                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
                Some(node)
            }
        }
    }
}
