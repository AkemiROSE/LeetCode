use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    helper(root, i64::MIN, i64::MAX)
  }
}


fn helper(root: Option<Rc<RefCell<TreeNode>>>, min_val: i64, max_val: i64) ->bool{
  if let Some(node) = root{
    let val = (*node).borrow().val as i64;
    if val <= min_val || val >= max_val {return false;}
    let mut tmp = (*node).borrow_mut();
    helper( tmp.left.take(),min_val, val) && helper(tmp.right.take(), val, max_val)
  }
  else{
    true
  }

}