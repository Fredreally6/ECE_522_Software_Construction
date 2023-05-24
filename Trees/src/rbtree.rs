use std::cell::RefCell;
use std::fmt::{Display,Debug};
use std::rc::{Rc, Weak};

pub use crate::basetree::{BaseTree, BaseTreeNode};

//Color of a red-black tree node
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    Red,
}


type RBNode<T> = Rc<RefCell<Node<T>>>;
//For children & root node
type OpNode<T> = Option<RBNode<T>>;

type WkNode<T> = Weak<RefCell<Node<T>>>;
//For parent
type OpWkNode<T> = Option<WkNode<T>>;

// Tree Node struct
#[derive(Clone, Debug)]
pub struct Node<T: Ord + Display> {
    pub key: T,
    pub color: Color,
    pub parent: OpWkNode<T>,
    pub left: OpNode<T>,
    pub right: OpNode<T>,
}
// The implement for base is here
impl  <T: Ord + Copy + Debug+std::fmt::Display>BaseTreeNode<T> for Node <T>  {
    fn get_left(&self) -> &Option<Rc<RefCell<Self>>> { return &self.left }
    fn get_right(&self) -> &Option<Rc<RefCell<Self>>> { return &self.right }
    fn get_data(&self) -> T { return self.key }
}

impl  <T: Ord + Copy + Debug+ std::fmt::Display> BaseTree<T,Node<T>> for RBTree<T> {
    fn get_root(&self) -> &OpNode<T> { &self.root}
}

// For tree node
impl<T: Ord + Display + Clone> Node<T> {
    fn new(c: Color, k: T) -> Self {
        Node {
            key: k,
            color: c,
            parent: None,
            left: None,
            right: None,
        }
    }

    //just insert the given node, do the fix with another function
    fn just_insert(node: &RBNode<T>, key: T) -> OpNode<T> {
        let cur_key = (*node.borrow()).key.clone();
        if key == cur_key {// doesn't insert the second node with same key
            None
        }
        else if key < cur_key {
            let left_child = &(*node.borrow()).left.clone();
            match left_child {
                //does not have left child, then create new node and insert (initial color is red)
                None => {
                    let new_node = Rc::new(RefCell::new(Node::new(Color::Red, key)));
                    (*node.borrow_mut()).left = Some(new_node.clone());
                    (*new_node.borrow_mut()).parent = Some(Rc::downgrade(&node));
                    Some(new_node.clone())
                }
                Some(to_insert) => {
                    Self::just_insert(to_insert, key)
                }
            }
        } else {
            let right_child = &(*node.borrow()).right.clone();
            match right_child {
                //does not have right child, then create new node and insert (initial color is red)
                None => {
                    let new_node = Rc::new(RefCell::new(Node::new(Color::Red, key)));
                    (*node.borrow_mut()).right = Some(new_node.clone());
                    (*new_node.borrow_mut()).parent = Some(Rc::downgrade(&node));
                    Some(new_node.clone())
                }
                Some(to_insert) => {
                    Self::just_insert(to_insert, key)
                }
            }
        }
    }

    //returns parent of the given node, None otherwise
    fn get_parent(node: &RBNode<T>) -> OpNode<T> {
        return if let Some(parent) = &(*node.borrow()).parent {
            parent.upgrade()
        } else {
            None
        }
    }

    //returns grandparent of the given node, None otherwise
    fn get_grandparent(node: &RBNode<T>) -> OpNode<T> {
        let parent = Self::get_parent(node);
        return if let Some(parent) = parent {
            Self::get_parent(&parent)
        } else {
            None
        }
    }

    //left rotation
    fn left_rotate(root: &mut RBNode<T>, node: &RBNode<T>) {
        //take right child of the node gotta be rotated
        let node_right = &(*node.borrow()).right.clone();
        //different conditions
        if let Some(node_right) = node_right {
            let node_parent = Self::get_parent(node);
            if let Some(node_right_left) = &(*node_right.borrow()).left {
                (*node.borrow_mut()).right = Some(node_right_left.clone());
                (*node_right_left.borrow_mut()).parent = Some(Rc::downgrade(&node));
            }
            else{
                (*node.borrow_mut()).right = None;
            }
            if let Some(node_parent) = node_parent {
                (*node_right.borrow_mut()).parent = Some(Rc::downgrade(&node_parent));
                if Self::is_left_child(node, &node_parent) {
                    (*node_parent.borrow_mut()).left = Some(node_right.clone());
                } else {
                    (*node_parent.borrow_mut()).right = Some(node_right.clone());
                }
            }
            else{
                (*node_right.borrow_mut()).parent = None;
                *root = node_right.clone(); // make node_right the root
            }
            (*node_right.borrow_mut()).left = Some(node.clone());
            (*node.borrow_mut()).parent = Some(Rc::downgrade(&node_right));
        }
    }

    //right rotation
    fn right_rotate(root: &mut RBNode<T>, node: &RBNode<T>) {
        //take left child of the node gotta be rotated
        let node_left = &(*node.borrow()).left.clone();
        if let Some(node_left) = node_left {
        //if node_left.is_some(){
            let node_parent = Self::get_parent(node);
            if let Some(node_left_right) = &(*node_left.borrow()).right {
            //if node_left_right.is_some(){
                (*node.borrow_mut()).left = Some(node_left_right.clone());
                (*node_left_right.borrow_mut()).parent = Some(Rc::downgrade(&node));
            }
            else{
                (*node.borrow_mut()).left = None;
            }
            if let Some(node_parent) = node_parent {
            //if node_parent.is_some(){
                (*node_left.borrow_mut()).parent = Some(Rc::downgrade(&node_parent));
                if Self::is_left_child(node, &node_parent) {
                    (*node_parent.borrow_mut()).left = Some(node_left.clone());
                } else {
                    (*node_parent.borrow_mut()).right = Some(node_left.clone());
                }
            }
            else{
                (*node_left.borrow_mut()).parent = None;
                *root = node_left.clone(); // set node_left as root
            }
            (*node_left.borrow_mut()).right = Some(node.clone());
            (*node.borrow_mut()).parent = Some(Rc::downgrade(&node_left));
        }
    }

    //fix function for insertion
    fn insert_fix(root: &mut RBNode<T>, node: &RBNode<T>) {
        let mut node_to_fix = node.clone();
        while let Some(mut p) = Self::get_parent(&node_to_fix) { //go in the iteration if current node has parent node
            if (*p.borrow()).color == Color::Black {// the color of parent node is black
                break;
                //println!("Parent Node is black, don't need to change color");
            }
            else {// the color of parent node is red
                let grandpa= Self::get_grandparent(&node_to_fix);
                if let Some(grandpa) = grandpa {
                    if Self::is_left_child(&p, &grandpa) { // if p is the left child of grandpa
                        let uncle = &(*grandpa.borrow()).right.clone();
                        if let Some(uncle) = uncle { // p is left, so uncle is right
                            if (*uncle.borrow()).color == Color::Red { // the color of uncle node is red
                                (*grandpa.borrow_mut()).color = Color::Red;
                                (*p.borrow_mut()).color = Color::Black;
                                (*uncle.borrow_mut()).color = Color::Black;
                                node_to_fix = grandpa.clone();// treat grandpa as new inserted node and go to next iteration
                                continue;
                            }// the color of uncle node is black
                        }
                        if !Self::is_left_child(&node_to_fix, &p) { //node is the right child of p
                            Self::left_rotate(root, &p);
                            let p2 = p.clone();
                            // treat p as the insert node and call function again
                            p = node_to_fix.clone();
                            node_to_fix = p2; // interchange p and n, let origin p as the new inserted node
                        }
                            //node is the left child of p
                        Self::right_rotate(root, &grandpa);
                        (*grandpa.borrow_mut()).color = Color::Red;
                        (*p.borrow_mut()).color = Color::Black;
                    }
                    else {// if p is the right child of grandpa
                        let uncle = &(*grandpa.borrow()).left.clone();
                        if let Some(uncle) = uncle {// p is right, so uncle is left
                            if (*uncle.borrow()).color == Color::Red { // the color of uncle node is red, same as above
                                (*grandpa.borrow_mut()).color = Color::Red;
                                (*p.borrow_mut()).color = Color::Black;
                                (*uncle.borrow_mut()).color = Color::Black;
                                node_to_fix = grandpa.clone();// treat grandpa as new inserted node and go to next iteration
                                continue;
                            }
                        }
                        // the color of uncle node is black
                        if Self::is_left_child(&node_to_fix, &p){//node is the left child of p
                            Self::right_rotate(root, &p);
                            let p2 = p.clone();
                            // treat p as the insert node and call function again
                            p = node_to_fix.clone();
                            node_to_fix = p2; // interchange p and n, let origin p as the new inserted node
                        }
                        //node is the right child of p
                        Self::left_rotate(root, &grandpa);
                        (*grandpa.borrow_mut()).color = Color::Red;
                        (*p.borrow_mut()).color = Color::Black;
                    }
                }
            }
        }
        (*root.borrow_mut()).color = Color::Black; // make sure the color of root Black
    }

    //delete function, calling related fix function inside
    #[allow(unused)]
    fn delete(root: &mut RBNode<T>, data: T){
        //let mut b = true;// fix or not
        let mut succ_right_child = 0; //if the node go into fixup function is a right child. 0 indicates that the node is not the right child of the deleting node.
        let node_to_delete = Self::search_node(root, data);
        if node_to_delete.is_none(){//if the node_to_delete doesn't exists
            return
        }
        // to next step if the node_to_delete exists
        if let Some(mut node_to_delete) = node_to_delete {
            let mut to_fix:RBNode<T> = node_to_delete.clone();// just initial
            let mut co_temp  = node_to_delete.borrow().color.clone();
            if node_to_delete.borrow().left.is_none() && node_to_delete.borrow().right.is_none(){ //don't have any children
                //to_fix = node_to_delete;
                if let Some(parent) = Self::get_parent(&to_fix) {
                    if Self::is_left_child(&to_fix, &parent) {
                        parent.borrow_mut().left = None;
                    } else {// leaf node is the right child
                        parent.borrow_mut().right = None;
                        succ_right_child = 1;
                    }
                }
            }
            //if node_to_delete.borrow().left.is_none() {
            else if node_to_delete.borrow().left.is_none() {// if left child of the deleting node doesn't exit, only has right child
                succ_right_child = 1;
                to_fix = node_to_delete.borrow().right.clone().unwrap();
                if let Some(parent) = Self::get_parent(&node_to_delete) {
                    Self::node_replace(&node_to_delete, &to_fix);
                }
                else {
                    *root = to_fix.clone();
                    (*to_fix.borrow_mut()).parent = None;
                }

            }
            else if node_to_delete.borrow().right.is_none(){// if right child of the deleting node doesn't exit, only has left child
                to_fix = node_to_delete.borrow().left.clone().unwrap();
                //Self::node_replace(&node_to_delete, &to_fix.unwrap(), root);
                if let Some(parent) = Self::get_parent(&node_to_delete) {
                    Self::node_replace(&node_to_delete, &to_fix);
                }
                else {
                    *root = to_fix.clone();
                    (*to_fix.borrow_mut()).parent = None;
                }
            }
            else{//both left and right exist
                let succ = Self::search_succ(&node_to_delete.borrow().right.clone().unwrap());
                if Rc::ptr_eq(&Self::get_parent(&succ).unwrap(), &node_to_delete) {
                    // if succ's parent is the deleting node, succ must be right child of the deleting node
                    succ_right_child = 1;
                }
                co_temp = succ.borrow().color.clone();
                //switch the values between deleting node and succ node, then take succ node as the deleting node.
                let temp = succ.borrow().key.clone();
                succ.borrow_mut().key = node_to_delete.borrow().key.clone();
                node_to_delete.borrow_mut().key = temp;
                to_fix = succ.clone();
                if succ.borrow().left.is_none() && succ.borrow().right.is_none(){
                    if let Some(parent) = Self::get_parent(&to_fix){
                        if Self::is_left_child(&to_fix, &parent) {
                            parent.borrow_mut().left = None;
                        }
                        else{
                            parent.borrow_mut().right = None;
                        }
                        //to_fix.borrow_mut().parent = None;
                    }
                }
                else{// succ has right child
                    to_fix = succ.borrow().right.clone().unwrap().clone();
                    if let Some(parent) = Self::get_parent(&succ) {
                        Self::node_replace(&succ, &to_fix);
                    }
                    else {
                        *root = to_fix.clone();
                        (*to_fix.borrow_mut()).parent = None;
                    }
                }
            }

            //only call fix function when the co_temp is black
            if co_temp == Color::Black {
                //let para = to_fix.clone();
                let mut parent = Self::get_parent(&to_fix);
                //Self::delete_fixup(Some(to_fix.clone()), root);
                Self::delete_fix(Some(to_fix.clone()), root, succ_right_child);
            }
        }
    }

    //update the new child and parent
    fn node_replace(node: &RBNode<T>, node_v: &RBNode<T>) {
        //node's parent exist
        let parent= Self::get_parent(&node).unwrap();
        if Self::is_left_child(&node, &parent){ //node is left_child
            (*parent.borrow_mut()).left = Some(node_v.clone());
        }
        else{// node is right_child
            (*parent.borrow_mut()).right = Some(node_v.clone());
        }
        let parent = Self::get_parent(&node).unwrap();
        (*node_v.borrow_mut()).parent = Some(Rc::downgrade(&parent));
    }

    //returns successor node
    fn search_succ(node: &RBNode<T>) -> RBNode<T>{
        let mut temp= node.clone();
        if (*node.borrow()).left.is_some(){
            loop {
                temp = (*node.borrow()).left.clone().unwrap();
                if temp.borrow().left.is_none(){
                    break;
                }
            }
        }
        return temp;
    }

    //fix the node if needed during deletion
    #[allow(unused)]
    fn delete_fix(node: OpNode<T>, root: &mut RBNode<T>, is_succ_right_child: i32){
        let mut is_current_right_child = is_succ_right_child;
        let mut current_node = node.clone().unwrap();
        while !Rc::ptr_eq(&current_node, root) && (*current_node.borrow()).color == Color::Black {
            //if let Some(parent) = parent.clone() {
            if let Some(parent) = Self::get_parent(&current_node) {
                //let parent2= Self::get_parent(&current_node).unwrap();
                //for debug
                //let c = current_node.borrow().key.clone();
                //let parent_key = parent.borrow().key.clone();
                if is_succ_right_child == 0 { // current node is left_child
                    //is_succ_right_child_temp = 2;
                    let mut sibling = parent.borrow().right.clone().unwrap();
                    if sibling.borrow().color == Color::Red {// if sibling's color is Red
                        sibling.borrow_mut().color = Color::Black;
                        parent.borrow_mut().color = Color::Red;
                        Self::left_rotate(root, &parent);
                        //parent = Self::get_parent(&current_node).unwrap();
                        sibling = parent.borrow().right.clone().unwrap();
                    }
                    //let sibling_left = sibling.borrow().left.clone().unwrap();
                    //let sibling_right = sibling.borrow().right.clone().unwrap();
                    // get color of sibling's children
                    let mut s_left_color;
                    let mut s_right_color;
                    let mut is_leaf_left = false;
                    let mut is_leaf_right = false;
                    if let Some(sibling_left) = sibling.borrow().left.clone(){
                        if sibling_left.borrow().color == Color::Black {
                            s_left_color = 0; // 0 denotes Black
                        }
                        else{
                            s_left_color = 1;// 1 denotes Red
                        }
                    }
                    else{
                        is_leaf_left = true;
                        s_left_color = 0;
                    }
                    if let Some(sibling_right) = sibling.borrow().right.clone(){
                        if sibling_right.borrow().color == Color::Black {
                            s_right_color = 0; // 0 denotes Black
                        }
                        else{
                            s_right_color = 1;// 1 denotes Red
                        }
                    }
                    else{
                        is_leaf_right = true;
                        s_right_color = 0;
                    }
                    if s_left_color == 0 && s_right_color == 0 { // both left and right of sibling are Black
                        sibling.borrow_mut().color = Color::Red;
                        //parent = Self::get_parent(&current_node).unwrap();
                        current_node = parent.clone();
                        continue;
                    }
                    else if s_right_color == 0 && s_left_color == 1 {//right is black and left is Red
                        if !is_leaf_left{
                            let sibling_left = sibling.borrow().left.clone().unwrap();
                            sibling_left.borrow_mut().color = Color::Black;
                        }
                        sibling.borrow_mut().color = Color::Red;
                        Self::right_rotate(root, &sibling);
                        //parent = Self::get_parent(&current_node).unwrap();
                        sibling = parent.borrow().right.clone().unwrap();
                    }
                    if s_right_color == 1 {
                        //parent = Self::get_parent(&current_node).unwrap();
                        sibling.borrow_mut().color = parent.borrow().color.clone();
                        if !is_leaf_right{
                            let sibling_right = sibling.borrow().right.clone().unwrap();
                            sibling_right.borrow_mut().color = Color::Black;
                        }
                        parent.borrow_mut().color = Color::Black;
                        Self::left_rotate(root, &parent);
                        current_node = root.clone();
                    }
                }
                else {//current node is right_child
                    let mut sibling = parent.borrow().left.clone().unwrap();
                    if sibling.borrow().color == Color::Red {// if sibling's color is Red
                        sibling.borrow_mut().color = Color::Black;
                        parent.borrow_mut().color = Color::Red;
                        Self::right_rotate(root, &parent);
                        //parent = Self::get_parent(&current_node).unwrap();
                        sibling = parent.borrow().left.clone().unwrap();
                    }
                    // get color of sibling's children
                    let mut s_left_color;
                    let mut s_right_color;
                    let mut is_leaf_left = false;
                    let mut is_leaf_right = false;
                    if let Some(sibling_left) = sibling.borrow().left.clone(){
                        if sibling_left.borrow().color == Color::Black {
                            s_left_color = 0; // 0 denotes Black
                        }
                        else{
                            s_left_color = 1;// 1 denotes Red
                        }
                    }
                    else{
                        is_leaf_left = true;
                        s_left_color = 0;
                    }
                    if let Some(sibling_right) = sibling.borrow().right.clone(){
                        if sibling_right.borrow().color == Color::Black {
                            s_right_color = 0; // 0 denotes Black
                        }
                        else{
                            s_right_color = 1;// 1 denotes Red
                        }
                    }
                    else{
                        is_leaf_right = true;
                        s_right_color = 0;
                    }
                    //let sibling_right = sibling.borrow().right.clone().unwrap();
                    if s_left_color == 0 && s_right_color == 0 { // both left and right of sibling are Black
                        sibling.borrow_mut().color = Color::Red;
                        //parent = Self::get_parent(&current_node).unwrap();
                        current_node = parent.clone();
                        continue;
                    }
                    else if s_left_color == 0 && s_right_color == 1 {//left is black and right is red
                        if !is_leaf_right{
                            let sibling_right = sibling.borrow().right.clone().unwrap();
                            sibling_right.borrow_mut().color = Color::Black;
                        }
                        sibling.borrow_mut().color = Color::Red;
                        Self::left_rotate(root, &sibling);
                        //parent = Self::get_parent(&current_node).unwrap();
                        sibling = parent.borrow().left.clone().unwrap();
                    }
                    if s_left_color == 1 { // left_child of cibling is red
                        //parent = Self::get_parent(&current_node).unwrap();
                        sibling.borrow_mut().color = parent.borrow().color.clone();
                        if !is_leaf_left{
                            let sibling_left = sibling.borrow().left.clone().unwrap();
                            sibling_left.borrow_mut().color = Color::Black;
                        }
                        parent.borrow_mut().color = Color::Black;
                        Self::right_rotate(root, &parent);
                        current_node = root.clone();
                        //break;
                    }
                }
            }
            else{
                break;
            }
            //update is_current_right_child given new current_node
            if let Some(cu_parent) = Self::get_parent(&current_node){ // if current node is not the root
                if !Self::is_left_child(&current_node, &cu_parent){
                    is_current_right_child = 1;
                }
            }
        }
        current_node.borrow_mut().color = Color::Black;// set the color of current_node Black
    }

    //judge if the given child node is the left child of the given preant node
    fn is_left_child(child: &RBNode<T>, parent: &RBNode<T>) -> bool {
        let left_child = &(*parent.borrow()).left;
        if let Some(left_child_node) = left_child {
            if (*child.borrow()).key == (*left_child_node.borrow()).key {
                return true;
            }
        }
        false
    }

    //given a value, return the node if there is a node with same key. None otherwise
    fn search_node(root: &RBNode<T>, data: T) -> OpNode<T> {
        if (*root.borrow()).key == data {
            return Some(root.clone());
        }
        return if (*root.borrow()).key <= data {
            let right = &(*root.borrow()).right;
            if let Some(right) = right {
                Self::search_node(right, data)
            } else {
                Option::None
            }
        } else {
            let left = &(*root.borrow()).left;
            if let Some(left) = left {
                Self::search_node(left, data)
            } else {
                Option::None
            }
        }
    }
}


//RBTree struct
#[derive(Debug)]
pub struct RBTree<T: Ord + Display> {
    pub root: OpNode<T>,
}

//For tree
impl<T: Ord + Display + Clone> RBTree<T> {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    //tree insert function
    pub fn insert(&mut self, data: T){
        match &mut self.root {
            Some(existing_root) => {
                let ret = Node::just_insert(existing_root, data);
                if let Some(new_node) = ret {
                    // if insert does happen
                    Node::insert_fix(existing_root,&new_node); // fix the tree
                }
            }
            //if the root is None, create a new tree node for the root
            None => {
                self.root = Some(Rc::new(RefCell::new(Node::new(Color::Black, data))));
            }
        }
    }

    //tree delete function
    pub fn delete(&mut self, data: T){
        match &mut self.root {
            Some(existing_node) => {
                if existing_node.borrow().key == data
                    && existing_node.borrow().left.is_none()
                    && existing_node.borrow().right.is_none(){ //deleting the root that doesn't have any children
                    self.root = None;
                }
                else{
                    Node::delete(existing_node, data);//fix function is contained in delete function
                }
            }
            None => return,//only conduct deletion when there has a root
        }
    }

    #[allow(dead_code)]
    pub fn search_node(&self, data: T) -> OpNode<T> {
        return match &self.root {
            Some(node) => Node::search_node(node, data),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //use std::intrinsics::raw_eq;
    //use std::cmp::PartialEq;

    #[test]
    //traverse this to a test
    fn test_basic_rbtree(){
        //test insert()
        let mut tree = RBTree::new();

        tree.insert(16);
        tree.insert(3);
        tree.insert(7);
        tree.insert(11);
        tree.insert(9);
        tree.insert(26);
        tree.insert(18);
        tree.insert(14);
        tree.insert(15);

        //println!("{:#?}", tree);
        //println!("\n\n");

        let mut tree2 = RBTree::new();
        tree2.insert(11);
        let t2_root = tree2.root.clone().unwrap();
        t2_root.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(Color::Red, 7))));
        let mut left = t2_root.borrow().left.clone().unwrap();
        left.borrow_mut().parent = Some(Rc::downgrade(&t2_root));
        t2_root.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(Color::Red, 18))));
        let mut right = t2_root.borrow().right.clone().unwrap();
        right.borrow_mut().parent = Some(Rc::downgrade(&t2_root));

        let mut t1 = left;
        let t2 = right;
        t1.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(Color::Black, 3))));
        left = t1.borrow().left.clone().unwrap();
        left.borrow_mut().parent = Some(Rc::downgrade(&t1));
        t1.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(Color::Black, 9))));
        right = t1.borrow().right.clone().unwrap();
        right.borrow_mut().parent = Some(Rc::downgrade(&t1));
        t2.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(Color::Black, 15))));
        left = t2.borrow().left.clone().unwrap();
        left.borrow_mut().parent = Some(Rc::downgrade(&t2));
        t2.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(Color::Black, 26))));
        right = t2.borrow().right.clone().unwrap();
        right.borrow_mut().parent = Some(Rc::downgrade(&t2));

        t1 = left;
        t1.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(Color::Red, 14))));
        left = t1.borrow().left.clone().unwrap();
        left.borrow_mut().parent = Some(Rc::downgrade(&t1));
        t1.borrow_mut().right = Some(Rc::new(RefCell::new(Node::new(Color::Red, 16))));
        right = t1.borrow().right.clone().unwrap();
        right.borrow_mut().parent = Some(Rc::downgrade(&t1));

        println!("{:#?}", tree2);
        tree.print_in_order();
        println!("\n");
        tree2.print_in_order();
        println!("\n");


        //test delete()
        //For ex1~4 & ex7~8 & ex10
        /*
        tree.insert(13);
        tree.insert(8);
        tree.insert(17);
        tree.insert(1);
        tree.insert(11);
        tree.insert(15);
        tree.insert(25);
        tree.insert(6);
        tree.insert(22);
        tree.insert(27);

        //ex1 PASS
        //tree.delete(6);
        //EX2 PASS
        //tree.delete(1);
        //ex3 PASS
        //tree.delete(17);
        //ex4 PASS
        //tree.delete(25);
        //ex7 PASS
        //tree.delete(13);
        //ex8 PASS
        //tree.delete(8);
        //ex10 PASS
        tree.delete(11);
         */

        //For ex5 & ex9
        /*
        tree.insert(7);
        tree.insert(3);
        tree.insert(18);
        tree.insert(10);
        tree.insert(22);
        tree.insert(8);
        tree.insert(11);
        tree.insert(26);
         */

        //ex5 PASS
        //tree.delete(18);
        //ex9 PASS
        //tree.delete(3);


        //For ex6
        /*
        tree.insert(5);
        tree.insert(2);
        tree.insert(8);
        tree.insert(1);
        tree.insert(4);
        tree.insert(7);
        tree.insert(9);
        tree.delete(2);

         */
        //println!("\n\n");
        println!("After deleting:");
        println!("{:#?}", tree);
        tree.print_in_order();
}
    #[test]
    //tbc is it necessary?
    fn test_insert(){}
    #[test]
    //tbc
    fn test_delete(){}
    #[test]
    fn test_node_exist(){
        let mut tree=RBTree::new();
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
        let mut tree=RBTree::new();
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
        assert_eq!(tree.count_leaves(),3);
    }
    #[test]
    fn test_tree_height(){
        let v=vec![8,6,9,4,5];
        let mut tree=RBTree::new();
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
        let mut tree=RBTree::new();
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
       tree.print_in_order();
    }
    #[test]
    fn test_is_empty(){
        let mut tree=RBTree::new();
        assert!(tree.is_empty());
        tree.insert(0);
        assert!(!tree.is_empty());
    }
    #[test]
    //nocapture
    fn test_print_tree(){
        let mut tree=RBTree::new();
        let v=vec![8,6,9,4,5];
        tree.insert(0);
        for i in v{
            tree.insert(i);
        }
        println!("{:?}", tree);
    }

}
