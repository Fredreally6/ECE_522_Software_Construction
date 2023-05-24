
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
use std::fmt;
use std::fmt::Debug;

//declare a trait for base node
pub trait BaseTreeNode<T: Ord + Copy + Debug> {

    fn get_left(&self) -> &Option<Rc<RefCell<Self>>>;
    fn get_right(&self) -> &Option<Rc<RefCell<Self>>>;
    fn get_data(&self) -> T;

//implement on nodes
//  3.count the number of leaves in a tree
    fn count_leaves(&self) -> usize {
       // if None==self.get_left() && None==self.get_right(){
         //   return 1
        //}
        if let Some(l)=self.get_left(){
            if let Some(r)=self.get_right(){
               return  l.borrow().count_leaves()+r.borrow().count_leaves()
                //No right child
            }else{ return l.borrow().count_leaves()}
        }else{//no left child
            if let Some(r)=self.get_right(){
                return r.borrow().count_leaves()
            }else { return 1 }
        }
    }

//4.Return the height of a tree.
    fn height(&self) -> usize {
        let mut left_height=0;
        let mut right_height=0;
        if let Some(l)=self.get_left() {
          left_height = l.borrow().height();
        }else{}
        if let Some(r)=self.get_right(){
             right_height=r.borrow().height();
        }else {}
     return max(left_height,right_height)+1
    }

    // 5.print in order traversal of the tree
    fn print_in_order(&self) {
        match self.get_left() {
            Some(l)=>l.borrow().print_in_order(),
            None=>{}
        }
        print!("{:?} ", self.get_data());
        match self.get_right() {
            Some(r)=>r.borrow().print_in_order(),
            None=>{}
        }
    }


//to check a node is exist or not.
    fn search(&self, val: T) -> bool {
        if self.get_data()==val{
            return true
            //value greater than the data
        }else if self.get_data()<val{
            if let Some(r)=self.get_right(){
                r.borrow().search(val)
            //value smaller than the data
            }else { return false }
        }else {
            if let Some(l)=self.get_left(){
                l.borrow().search(val)
            }else { return false }
            }
    }
//to get the len of the tree
    #[allow(unused)]
    fn len(&self) -> usize {
        let mut left_len=0;
        let mut right_len=0;
        if let Some(l)=self.get_left(){
            left_len=l.borrow().len();
        }else { left_len=0; }
        if let Some(r)=self.get_right(){
            right_len=r.borrow().len();
        }else { right_len=0; }
        return  left_len+right_len+1
    }

    //for benchmark:
    //return the smallest node in the tree
    fn get_min(&self) -> T {
        let mut min=self.get_data();
        if let Some(l)=self.get_left(){
            min=l.borrow().get_min();
        }else {}
        return min
    }

    //return the largest node in the tree
    fn get_max(&self) -> T {
        let mut max=self.get_data();
        if let Some(l)=self.get_right(){
            max=l.borrow().get_max();
        }else {}
        return max
    }
}

// implement for base tree
pub trait BaseTree<T: Ord + Copy + fmt::Debug, B: BaseTreeNode<T>> {
    fn get_root(&self) -> &Option<Rc<RefCell<B>>>;

//use method in node, recursively visiting the tree.
    fn count_leaves(&self) -> usize {
        if let Some(root) = self.get_root(){
          let nums=root.borrow().count_leaves();
          return nums
        }else{
            return 0
     }
    }
//to get the height of the tree
    fn height(&self) -> usize {
    if let Some(root)=self.get_root(){
        let height=root.borrow().height();
        height
    }else{
        return 0
    }
    }

//5.to print in order traverse
    fn print_in_order(&self) {
        if let Some(n)=&self.get_root(){
             println!("Print in_order traverse:");
             n.borrow().print_in_order();
        }else{
                println!("The tree is empty!");
        }

    }
//6.check if a tree is empty or not
    fn is_empty(&self) -> bool {
        return if let Some(_root) = self.get_root() {
            false
        } else { true }

    }

   fn search(&self, val: T) -> bool {
       return if let Some(root) = self.get_root() {
           let bool = root.borrow().search(val);
           bool
       } else { false }

    }

    fn len(&self) -> usize {
        if let Some(root)=self.get_root(){
            let len=root.borrow().len();
            return len
        }else{
            return 0
        }

    }

//for benchmark:
    fn get_min(&self) -> Option<T> {
    if let Some(root)=self.get_root(){
        let min=root.borrow().get_min();
            return Some(min)
    }else {None}

    }

    fn get_max(&self) -> Option<T> {
        if let Some(root)=self.get_root(){
            let max=root.borrow().get_max();
            return Some(max)
        }else {None}
    }

}
