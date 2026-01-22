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

// NOTE: this solution uses Inorder Traversal Algorithm
//  we first go to the left as deep as possible, and then
//  we start going right, this gives us strick order where
//  we get a properly ordered array, and we can easily find the
//  problematic numbers
pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let c1 = &mut None;
    let c2 = &mut None;
    dfs(root, &mut None, c1, c2);

    if let (Some(c1_rc), Some(c2_rc)) = (c1, c2) {
        let mut c1_r = c1_rc.borrow_mut();
        let mut c2_r = c2_rc.borrow_mut();
        let tmp = c1_r.val;
        c1_r.val = c2_r.val;
        c2_r.val = tmp;
    }
}

fn dfs(
    node: &Option<Rc<RefCell<TreeNode>>>,
    p: &mut Option<Rc<RefCell<TreeNode>>>,
    c1: &mut Option<Rc<RefCell<TreeNode>>>,
    c2: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(node_rc) = node {
        let (n_val, n_left, n_right) = {
            let n = node_rc.borrow();
            (n.val, n.left.clone(), n.right.clone())
        };

        dfs(&n_left, p, c1, c2);

        if c1.is_none() && let Some(p_rc) = p && p_rc.borrow().val >= n_val {
            *c1 = Some(p_rc.clone());
        }
        if c1.is_some() && let Some(p_rc) = p && p_rc.borrow().val >= n_val {
            *c2 = Some(node_rc.clone());
        }
        *p = Some(node_rc.clone());

        dfs(&n_right, p, c1, c2);
    }
}

//  1 2 3 4 7 12 9 10 11 17 8 20  |   3 2 1        |   1 3* 2 4
//               7                |        1        |        3       |        6
//         3           9          |     3     -     |     1     4    |     1     7
//      1     4   12      11      |   -   2 -   -   |   -   - 2   -  |          3
//     - 2   - - - -  10    17    |                 |                |         5
//                   -  -  8  20  |                 |                |

#[test]
fn test() {
    let one = &mut Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: None,
    })));
    let one_ans = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))),
        right: None,
    })));
    recover_tree(one);
    assert_eq!(*one, one_ans);

    let two = &mut Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        }))),
    })));
    let two_ans = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));
    recover_tree(two);
    assert_eq!(*two, two_ans);

    let sixteen = &mut Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483647,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-2147483648)))),
    })));
    let sixteen_ans = Some(Rc::new(RefCell::new(TreeNode {
        val: -2147483648,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    recover_tree(sixteen);
    assert_eq!(*sixteen, sixteen_ans);
}
