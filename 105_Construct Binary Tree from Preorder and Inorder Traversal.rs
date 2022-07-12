use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() == 0 {return None;}
    helper(&preorder[..], &inorder[..])
  }
}

fn helper(preorder: &[i32], inorder: &[i32] ) -> Option<Rc<RefCell<TreeNode>>> {
  let root = preorder[0];
  let root_index = inorder.to_vec().iter().position(|&item| item == root).unwrap();
  let mut root = TreeNode::new(root);
  let leftlen = root_index;
  let rightlen = inorder.len() - 1 - root_index;
  if leftlen > 0 {root.left = helper(&preorder[1..1+leftlen], &inorder[0..root_index]);}
  if rightlen > 0 {root.right = helper(&preorder[1+leftlen..], &inorder[root_index+1..]);}

  Some(Rc::new(RefCell::new(root)))
}