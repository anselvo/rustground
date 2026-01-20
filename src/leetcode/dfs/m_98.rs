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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dfs(root, None, None)
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, l: Option<i32>, r: Option<i32>) -> bool {
    if root.is_none() { return true }
    let node_rc = root.unwrap();
    let node = node_rc.borrow();
    if let Some(lt) = l && lt >= node.val { return false }
    if let Some(rt) = r && rt <= node.val { return false }
    dfs(node.left.clone(), l, Some(node.val)) && dfs(node.right.clone(), Some(node.val), r)
}

//             7
//        3          9
//     1     4    8      11
//       2            10    12
//
//   2
// 2   2
//

#[test]
fn test() {
    let sixteen = Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483647,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    assert_eq!(is_valid_bst(sixteen), false);
    let fifteen = Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483647,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
        right: None,
    })));
    assert_eq!(is_valid_bst(fifteen), false);
    let fourteenth = Some(Rc::new(RefCell::new(TreeNode {
        val: -2147483648,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-2147483648)))),
    })));
    assert_eq!(is_valid_bst(fourteenth), false);
    let thirteen = Some(Rc::new(RefCell::new(TreeNode {
        val: -2147483648,
        left: Some(Rc::new(RefCell::new(TreeNode::new(-2147483648)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    assert_eq!(is_valid_bst(thirteen), false);
    let twelve = Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483647,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    assert_eq!(is_valid_bst(twelve), false);
    let eleven = Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483646,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2147483645)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    assert_eq!(is_valid_bst(eleven), true);
    let ten = Some(Rc::new(RefCell::new(TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode::new(-2147483648)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    assert_eq!(is_valid_bst(ten), true);
    let nine = Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483647,
        left: Some(Rc::new(RefCell::new(TreeNode::new(-2147483648)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
    })));
    assert_eq!(is_valid_bst(nine), false);
    let eight = Some(Rc::new(RefCell::new(TreeNode::new(-2147483648))));
    assert_eq!(is_valid_bst(eight), true);
    let seven = Some(Rc::new(RefCell::new(TreeNode::new(2147483647))));
    assert_eq!(is_valid_bst(seven), true);
    let six = Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
            }))),
        }))),
    })));
    assert_eq!(is_valid_bst(six), true);
    let five = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(is_valid_bst(five), false);
    let four = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    assert_eq!(is_valid_bst(four), false);
    let one = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(is_valid_bst(one), true);
    let two = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));
    assert_eq!(is_valid_bst(two), false);
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(is_valid_bst(tree), false);
    let six = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        }))),
    })));
    assert_eq!(is_valid_bst(six), true);
    assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(TreeNode::new(2))))), true);
}
