use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {return 0;}
    let mut res = 0;
    helper(root.unwrap(), 0, &mut res);
    res
  }
}
fn helper(root: Rc<RefCell<TreeNode>>, mut tmp:i32, res: &mut i32) {
  let node = (*root).borrow();
  tmp = tmp * 10 + node.val;
  if node.left.is_none() && node.right.is_none(){
    *res += tmp;
  }
  else{
    if node.left.is_some() {helper(node.left.as_ref().unwrap().clone(), tmp, res);}
    if node.right.is_some() {helper(node.right.as_ref().unwrap().clone(), tmp, res);}
  }
}