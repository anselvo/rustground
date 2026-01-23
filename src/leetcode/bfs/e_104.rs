use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0 }
    let root_rc = root.unwrap();
    let mut stack = VecDeque::new();
    stack.push_back(root_rc);
    let mut count_depth = 0;
    while !stack.is_empty() {
        count_depth += 1;
        for _ in 0..stack.len() {
            let node = stack.pop_front().unwrap();
            let nr = node.borrow();
            if let Some(l) = nr.left.clone() { stack.push_back(l); }
            if let Some(r) = nr.right.clone() { stack.push_back(r); }
        }
    }
    count_depth
}


#[test]
fn test() {
    let one = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 9,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        }))),
        right: None,
    })));
    assert_eq!(max_depth(one), 7);
    let two = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(max_depth(two), 3);
    let sixteen = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    assert_eq!(max_depth(sixteen), 3);
    let fifteen = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));
    assert_eq!(max_depth(fifteen), 3);
    let fourteenth = Some(Rc::new(RefCell::new(TreeNode {
        val: -100,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-100)))),
    })));
    assert_eq!(max_depth(fourteenth), 2);
    let thirteen = Some(Rc::new(RefCell::new(TreeNode {
        val: -100,
        left: Some(Rc::new(RefCell::new(TreeNode::new(100)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(100)))),
    })));
    assert_eq!(max_depth(thirteen), 2);
    assert_eq!(max_depth(Some(Rc::new(RefCell::new(TreeNode::new(2))))), 1);
    assert_eq!(max_depth(None), 0);
}