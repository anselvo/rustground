use std::cell::RefCell;
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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p == None && q == None { return true }
    if p != q { return false }
    let p_rc = p.unwrap();
    let node_p = p_rc.borrow();
    let q_rc = q.unwrap();
    let node_q = q_rc.borrow();
    if node_q.val != node_p.val { return false }
    is_same_tree(node_p.left.clone(), node_q.left.clone()) && is_same_tree(node_p.right.clone(), node_p.right.clone())
}

#[test]
fn test() {
    let one = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(is_same_tree(one.clone(), one.clone()), true);
    let two_p = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let two_q = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(is_same_tree(two_p, two_q), false);
    let three_p = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    })));
    let three_q = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(is_same_tree(three_p, three_q), false);
    assert_eq!(is_same_tree(None, None), true);
}
