//mod two_pointers;

use crate::dfs::max_depth::TreeNode;
use std::{
    rc::Rc,
    cell::RefCell,
};
mod dfs;

fn main() {
    let results = TreeNode::max_depth(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
            }))),
        }))));
    println!("{:?}", results);
    
    println!("Successful");
}
