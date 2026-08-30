use std::{
    rc::Rc,
    cell::RefCell,
    cmp::max,
};
#[derive(PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn max_depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // we create the base case upon which recursion should end.

        if let Some(n) = node {
            let borrowed = n.borrow();
            let left_depth = Self::max_depth(borrowed.left.clone());
            let right_depth = Self::max_depth(borrowed.right.clone());

            1 + max(left_depth, right_depth)
        } else {
            0
        }

    }
}
