use std::{
    rc::Rc,
    cell::RefCell,
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
        if node == None {
            return 0;
        }


        
        0
    }
}
