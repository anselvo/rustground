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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root_rc = root.unwrap();
    let root_r = root_rc.borrow();
    bfs(root_r.left.clone(), root_r.right.clone())
}

fn bfs(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![l, r];
    while stack.len() > 0 {
        let el_l = stack.pop().unwrap();
        let el_r = stack.pop().unwrap();
        if el_l.is_some() != el_r.is_some() { return false }
        if let Some(lt) = el_l && let Some(rt) = el_r {
            let l_rc = lt.borrow();
            let r_rc = rt.borrow();
            if l_rc.val != r_rc.val { return false }
            stack.push(l_rc.left.clone());
            stack.push(r_rc.right.clone());
            stack.push(l_rc.right.clone());
            stack.push(r_rc.left.clone());
        }
    }
    true
}


#[test]
fn test() {
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
    assert_eq!(is_symmetric(sixteen), true);
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
    assert_eq!(is_symmetric(fifteen), false);
    let fourteenth = Some(Rc::new(RefCell::new(TreeNode {
        val: -100,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-100)))),
    })));
    assert_eq!(is_symmetric(fourteenth), false);
    let thirteen = Some(Rc::new(RefCell::new(TreeNode {
        val: -100,
        left: Some(Rc::new(RefCell::new(TreeNode::new(100)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(100)))),
    })));
    assert_eq!(is_symmetric(thirteen), true);
    assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(TreeNode::new(2))))), true);
}