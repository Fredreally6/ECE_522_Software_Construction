
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::fmt;
use std::fmt::{Debug, Display};
pub use crate::basetree::{BaseTree,BaseTreeNode};
///for benchmark.
type BSNode<T>=Rc<RefCell<Node<T>>>;
pub type BSTree<T>=Option<BSNode<T>>;
#[derive(Debug)]
pub struct Node<T: Ord + Copy + Debug+Display> {
    data: T,
    left_child: BSTree<T>,
    right_child:BSTree<T>,
}

impl <T: Ord + Copy + Debug + Display> BaseTreeNode<T> for Node<T>  {
    fn get_left(&self) -> &BSTree<T> {
        return &self.left_child;
    }
    fn get_right(&self) -> &BSTree<T> {
        return &self.right_child;
    }
    fn get_data(&self) -> T {
        return self.data;
    }
}


impl <T:Ord + Copy + Debug + Display> Node<T> {

    pub fn new(data:T)->Node<T>{
        return Node {
            data,
            left_child: None,
            right_child: None,
        }

    }

    fn insert(&mut self, data: T) {
        if self.data == data {
            return
        }
        let new_node = if data < self.data {&mut self.left_child} else {&mut self.right_child};
        match new_node {
            Some(node) => node.borrow_mut().insert(data),
            None => {
                let that_node=Node{data,left_child:None,right_child:None};
                let boxed_node=Some(Rc::new(RefCell::new(that_node)));
                *new_node =boxed_node;
            }
        }
    }
}
//implement this to use the method in basetree.
pub struct BSRootTree<T: Ord + Copy + fmt::Debug+ Display>{
    root:BSTree<T>
}
impl <T: Ord + Copy + Debug + Display> BaseTree<T, Node<T>> for BSRootTree<T>  {
    fn get_root(&self) -> &Option<Rc<RefCell<Node<T>>>> {
        return &self.root
    }
}

impl <T:Ord + Copy + Debug + Display> BSRootTree<T>{
    #[allow(dead_code)]
  pub fn new()->BSRootTree<T>{
      BSRootTree{
          root:None
      }
  }
    //insert based on the tree
    #[allow(dead_code)]
   pub fn insert(&mut self, data:T){
        if let Some(root)=self.get_root(){
            root.borrow_mut().insert(data);
        }else{
            self.root=Some(Rc::new(RefCell::new(Node::new(data))));
        }
    }
}



//not work well
/*impl <T:Ord + Copy + Debug + Display> BSRootTree<T>{
    pub fn new() ->BSTree<T>{
        let tree =BSRootTree{root:None};
        return tree.root
    }
}*/





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut tree = Node { data: 5, left_child: None, right_child: None };
        tree.insert(1);
        tree.insert(6);
        tree.insert(7);
        tree.insert(3);
        print!("The tree seems like:{:?}", tree);
    }
    #[test]
    fn test_print_in_order() {
        let mut tree = Node { data: 5, left_child: None, right_child: None };
        tree.insert(1);
        tree.insert(6);
        tree.insert(7);
        tree.insert(3);
        tree.print_in_order();
    }
    #[test]
    fn test_height(){
        let mut tree = Node { data: 5, left_child: None, right_child: None };
        tree.insert(1);
        tree.insert(6);
        tree.insert(7);
        tree.insert(3);
        println!("height of tree is {}",tree.height());
    }

}

