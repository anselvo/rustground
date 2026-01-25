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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![] }
    let root_rc = root.unwrap();
    let mut stack = VecDeque::new();
    stack.push_back(root_rc);
    let mut ans: Vec<Vec<i32>> = vec![];
    while !stack.is_empty() {
        let mut vec = vec![];
        for _ in 0..stack.len() {
            let el = stack.pop_front().unwrap();
            let el_r = el.borrow_mut();
            vec.push(el_r.val);
            if let Some(l) = &el_r.left { stack.push_back(l.clone()); }
            if let Some(r) = &el_r.right { stack.push_back(r.clone()); }
        }
        ans.push(vec);
    }
    ans
}


#[test]
fn test() {
    let two = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
        }))),
    })));
    assert_eq!(level_order(two), vec![vec![3], vec![9, 20], vec![15, 17]]);
    assert_eq!(level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))), vec![vec![1]]);
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
    assert_eq!(level_order(one), vec![vec![3], vec![9], vec![9], vec![9], vec![9], vec![9], vec![15]]);
    let fourteenth = Some(Rc::new(RefCell::new(TreeNode {
        val: -100,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-100)))),
    })));
    assert_eq!(level_order(fourteenth), vec![vec![-100], vec![-100]]);
    let thirteen = Some(Rc::new(RefCell::new(TreeNode {
        val: -100,
        left: Some(Rc::new(RefCell::new(TreeNode::new(100)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(100)))),
    })));
    assert_eq!(level_order(thirteen), vec![vec![-100], vec![100, 100]]);
    assert_eq!(level_order(Some(Rc::new(RefCell::new(TreeNode::new(2))))), vec![vec![2]]);
    assert_eq!(level_order(None), Vec::<Vec<i32>>::new());
}