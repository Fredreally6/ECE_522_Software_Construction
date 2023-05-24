
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::{Ord};
use std::fmt;

pub use crate::basetree::{BaseTreeNode, BaseTree};


type AVLTreeNode<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[derive(Debug, Clone)]
pub struct TreeNode<T:Ord + Copy + fmt::Debug+fmt::Display>{
    pub val: T,
    height: usize,
    left: AVLTreeNode<T>,
    right: AVLTreeNode<T>
}
#[derive(Debug)]
pub struct AVLTree<T:Ord + Copy + fmt::Debug+fmt::Display>{
    root: AVLTreeNode<T>
}

impl <T: Ord + Copy + fmt::Debug+fmt::Display> BaseTreeNode<T> for TreeNode<T> {
    fn get_left(&self) -> &AVLTreeNode<T> { return &self.left; }
    fn get_right(&self) -> &AVLTreeNode<T> { return &self.right; }
    fn get_data(&self) -> T { return self.val; }
}

impl <T: Ord + Copy + fmt::Debug+fmt::Display> BaseTree<T, TreeNode<T>> for AVLTree<T> {
    fn get_root(&self) -> &AVLTreeNode<T> {
        &self.root
    }
}

impl <T: Ord + Copy + fmt::Debug+fmt::Display> TreeNode<T> {
    fn new(val: T) -> AVLTreeNode<T>{
        Some(Rc::new(RefCell::new(Self {
            val,
            height: 1,
            left: None,
            right: None
        })))
    }

    #[allow(unused_mut)]
    #[allow(dead_code)]
    fn get_height(node: AVLTreeNode<T>) -> usize {
        let mut height = 0;
        if let Some(h) = node {
            height = h.borrow().height();
        }else{}
        return height;
    }

    #[allow(unused_mut)]
    #[allow(dead_code)]
    fn is_balanced(&self) -> bool {
        let mut left_height=0;
        let mut right_height=0;
        if let Some(l) = self.get_left() {
            left_height = l.borrow().height();
        }else{}
          if let Some(r) = self.get_right(){
            right_height = r.borrow().height();
        }else {}
        let height_diff = left_height as i32 - right_height as i32;
        if height_diff.abs() > 1 { //if the height difference between two node-trees is bigger than 1, it is not balanced
            return false;
        }

        else {
            let mut is_l_balanced = false;
            let mut is_r_balanced = false;
            if let Some(l) = self.get_left() {
                if l.borrow().is_balanced() {
                    is_l_balanced = true;
                }
            }else {}
            if let Some(r) = self.get_right() {
                if r.borrow().is_balanced() {
                    is_r_balanced = true;
                }
            }else {}
            if is_l_balanced && is_r_balanced {
                return true;
            }
            else {
                return false;
            }
        }
    }

    #[allow(unused_mut)]
    fn rotate_left(mut root: Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        // single left rotate
        let mut root_rotated = root.borrow().right.clone().unwrap();
        root.borrow_mut().right = root_rotated.borrow().left.clone().take();
        let mut left_height=0;
        let mut right_height=0;
        if let Some(l) = root.borrow().get_left() {
            left_height = l.borrow().height();
        }else{}
          if let Some(r) = root.borrow().get_right(){
            right_height = r.borrow().height();
        }else {}
        let mut new_height;
        if left_height > right_height {
            new_height = left_height + 1;
        }
        else {
            new_height = right_height + 1;
        }
        root.borrow_mut().height = new_height;
        root_rotated.borrow_mut().left = Some(root);
        let mut new_left_height=0;
        let mut new_right_height=0;
        if let Some(l) = root_rotated.borrow().get_left() {
            new_left_height = l.borrow().height();
        }else{}
          if let Some(r) = root_rotated.borrow().get_right(){
            new_right_height = r.borrow().height();
        }else {}
        if new_left_height > new_right_height {
            new_height = new_left_height + 1;
        }
        else {
            new_height = new_right_height + 1;
        }
        root_rotated.borrow_mut().height = new_height;
        return root_rotated;
    }

    #[allow(unused_mut)]
    fn rotate_right(mut root: Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        // single right rotate
        let mut root_rotated = root.borrow().left.clone().unwrap();
        root.borrow_mut().left = root_rotated.borrow().right.clone().take();
        let mut left_height=0;
        let mut right_height=0;
        if let Some(l) = root.borrow().get_left() {
            left_height = l.borrow().height();
        }else{}
          if let Some(r) = root.borrow().get_right(){
            right_height = r.borrow().height();
        }else {}
        let mut new_height;
        if left_height > right_height {
            new_height = left_height + 1;
        }
        else {
            new_height = right_height + 1;
        }
        root.borrow_mut().height = new_height;
        root_rotated.borrow_mut().right = Some(root);
        let mut new_left_height=0;
        let mut new_right_height=0;
        if let Some(l) = root_rotated.borrow().get_left() {
            new_left_height = l.borrow().height();
        }else{}
          if let Some(r) = root_rotated.borrow().get_right(){
            new_right_height = r.borrow().height();
        }else {}
        if new_left_height > new_right_height {
            new_height = new_left_height + 1;
        }
        else {
            new_height = new_right_height + 1;
        }
        root_rotated.borrow_mut().height = new_height;
        return root_rotated;
    }
    #[allow(unused_mut)]
    fn rotate_l_r(mut root: Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        // left_right rotate, first do single left rotate to left node, then do single right rotate to root node
        let l = root.borrow().left.clone().take().unwrap();
        root.borrow_mut().left = Some(Self::rotate_left(l));
        return Self::rotate_right(root);
    }
    #[warn(unused_mut)]
    fn rotate_r_l( root: Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        // right_left rotate, first do single right rotate to right node, then do single left rotate to root node
        let r = root.borrow().right.clone().take().unwrap();
        root.borrow_mut().right = Some(Self::rotate_right(r));
        return Self::rotate_left(root);
    }

    fn insert(node: AVLTreeNode<T>, val: T) -> AVLTreeNode<T> {
        let new_node = match node {
            Some(x) =>{
                let nval = x.borrow().val;
                if val > nval {
                    let right = x.borrow().right.clone();
                    x.borrow_mut().right = Self::insert(right, val);
                }
                else if val < nval {
                    let left = x.borrow().left.clone();
                    x.borrow_mut().left = Self::insert(left, val);
                }
                x
            },
            None => TreeNode::new(val).unwrap()
        };
        // adjusting balance
        let mut left_height=0;
        let mut right_height=0;
        if let Some(l) = new_node.borrow().get_left() {
            left_height = l.borrow().height();
        }else{}
          if let Some(r) = new_node.borrow().get_right(){
            right_height = r.borrow().height();
        }else {}
        let height_diff = left_height as i32 - right_height as i32;
        let new_node = if height_diff == 2 {
            if new_node.borrow().left.clone().unwrap().borrow().val > val {
                Self::rotate_right(new_node)
            }
            else {
                Self::rotate_l_r(new_node)
            }
        }
        else if height_diff == -2 {
            if new_node.borrow().right.clone().unwrap().borrow().val < val {
                Self::rotate_left(new_node)
            }
            else {
                Self::rotate_r_l(new_node)
            }
        }
        else {
            new_node
        };
        let mut new_left_height=0;
        let mut new_right_height=0;
        if let Some(l) = new_node.borrow().get_left() {
            new_left_height = l.borrow().height();
        }else{}
          if let Some(r) = new_node.borrow().get_right(){
            new_right_height = r.borrow().height();
        }else {}
        let new_height;
        if new_left_height > new_right_height {
            new_height = new_left_height + 1;
        }
        else {
            new_height = new_right_height + 1;
        }
        new_node.borrow_mut().height = new_height;
        return Some(new_node);
    }

    fn delete(node: AVLTreeNode<T>, val:T) -> AVLTreeNode<T> {
        let del_node = match node {
            Some(x) => {
                let nval = x.borrow().val;
                if nval == val { //three situations and deal with each one
                    let left = x.borrow().left.clone();
                    let right = x.borrow().right.clone();
                    let del = match (left.clone(), right.clone()){
                       // #[warn(unused_variables)]
                        (Some(_l), Some(r)) => { // both left and right child-node exist, choose the smaller one and replace it
                            let minval = r.borrow().get_min();
                            x.borrow_mut().val = minval;
                            let right = x.borrow().right.clone().take();
                            x.borrow_mut().right = Self::delete(right, minval);
                            Some(x)
                        }
                        (Some(l),_) => Some(l), // only left child exists, so replace it with left child
                        (_,Some(r)) => Some(r), // only right child exists, so replace it with right child
                        (_,None) => None,       // no child exists, so delete it by replacing it with None
                    };
                    del
                }
                else if val < nval {
                    // go left by recursing
                    let left = x.borrow().left.clone();
                    if !left.is_none() {
                        let l = x.borrow().left.clone().take();
                        x.borrow_mut().left = Self::delete(l, val);
                    }
                    else {
                        return Some(x)
                    }
                    Some(x)
                }
                else {
                    //go right by recursing
                    let right = x.borrow().right.clone();
                    if !right.is_none() {
                        let r = x.borrow().right.clone().take();
                        x.borrow_mut().right = Self::delete(r, val);
                    }
                    else {
                        return Some(x)
                    }
                    Some(x)
                }
            },
            None => node
        };
        //adjusting balance
        match del_node {
            Some(x) => {
                let mut left_height=0;
                let mut right_height=0;
                if let Some(l) = x.borrow().get_left() {
                    left_height = l.borrow().height();
                }else{}
                  if let Some(r) = x.borrow().get_right(){
                    right_height = r.borrow().height();
                }else {}
                let height_diff = left_height as i32 - right_height as i32;
                let del_n = if height_diff == 2 {
                    let l = &x.borrow().left.clone().unwrap();
                    let mut ll_height=0;
                    let mut lr_height=0;
                    if let Some(l) = l.borrow().get_left() {
                        ll_height = l.borrow().height();
                    }else{}
                      if let Some(r) = l.borrow().get_right(){
                        lr_height = r.borrow().height();
                    }else {}
                    if ll_height >= lr_height {
                        Self::rotate_right(x)
                    }
                    else {
                        Self::rotate_l_r(x)
                    }
                }
                else if height_diff == -2 {
                    let r = &x.borrow().right.clone().unwrap();
                    let mut rl_height=0;
                    let mut rr_height=0;
                    if let Some(l) = r.borrow().get_left() {
                        rl_height = l.borrow().height();
                    }else{}
                      if let Some(r) = r.borrow().get_right(){
                        rr_height = r.borrow().height();
                    }else {}
                    if rr_height < rl_height {
                        Self::rotate_r_l(x)
                    }
                    else {
                        Self::rotate_left(x)
                    }
                }
                else {
                    x
                };
                let new_height;
                let mut new_left_height=0;
                let mut new_right_height=0;
                if let Some(l) = del_n.borrow().get_left() {
                    new_left_height = l.borrow().height();
                }else{}
                  if let Some(r) = del_n.borrow().get_right(){
                    new_right_height = r.borrow().height();
                }else {}
                if new_left_height > new_right_height {
                    new_height = new_left_height + 1;
                }
                else {
                    new_height = new_right_height + 1;
                }
                del_n.borrow_mut().height = new_height;
                Some(del_n)
            },
            None => del_node
        }
    }
}

impl<T: Ord + Copy + fmt::Debug+std::fmt::Display> AVLTree<T> {
    pub fn new() -> Self{
        Self {root: None}
    }

    pub fn insert(&mut self, val: T) {
        match self.root.take() {
            Some(x) => {
                self.root = TreeNode::insert(Some(x), val)
            },
            None => self.root = TreeNode::new(val)
        }
    }

    pub fn delete(&mut self, val: T) {
        match self.root.take() {
            Some(x) => {
                self.root = TreeNode::delete(Some(x), val)
            },
            None => return
        }
    }

    fn _is_balanced(&self) -> bool{
        match self.get_root() {
            Some(x) => x.borrow().is_balanced(),
            None => true
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_basic_avl() {
        let mut avl = AVLTree::<i32>::new();
        assert_eq!(avl.height(), 0);
        assert_eq!(avl.is_empty(), true);
        assert_eq!(avl.len(), 0);

        for a in vec![1, 0, 2, 3, 4, 5, 10, 6, 9] {
            avl.insert(a);
            avl.print_in_order();
        }
        assert_eq!(avl.len(), 9);
        assert_eq!(avl.is_empty(), false);
        assert_eq!(avl.height(), 4);
        assert_eq!(avl.search(2),true);
        assert_eq!(avl.search(8),false);
        assert_eq!(avl.get_min().unwrap(),0);
        assert_eq!(avl.get_max().unwrap(),10);
    }
   #[test]
    fn test_node_exist(){
        let mut tree=AVLTree::new();
        tree.insert(0);
        let v=vec![8,6,9,11,12,13];
        for i in v{
            tree.insert(i);
        }
        //not exist in tree:
        assert!(!tree.search(7));
        assert!(tree.search(8));

    }
    #[test]

    fn test_count_node(){
        let v=vec![8,6,9,4,5];
        let mut tree=AVLTree::new();
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
        assert_eq!(tree.count_leaves(),3);
    }
    #[test]
    fn test_tree_height(){
        let v=vec![8,6,9,4,5];
        let mut tree=AVLTree::new();
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
        assert_eq!(tree.height(),3);
    }
    #[test]
    //nocapture
    fn test_print_in_order(){
        let v=vec![8,6,9,4,5];
        let mut tree=AVLTree::new();
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
        tree.print_in_order();
    }
    #[test]
    fn test_is_empty(){
        let mut tree=AVLTree::new();
        assert!(tree.is_empty());
        tree.insert(0);
        assert!(!tree.is_empty());
    }
    #[test]
    //nocapture
    fn test_print_tree(){
        let mut tree=AVLTree::new();
        let v=vec![8,6,9,4,5];
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
        println!("{:?}", tree);
    }


}
