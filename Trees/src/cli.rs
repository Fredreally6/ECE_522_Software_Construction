use crate::basetree::BaseTree;
use crate::rbtree::RBTree;
use crate::avltree::AVLTree;
//use std::io::{stdin, stdout, Write};

pub fn get_input()->String{
    let mut input=String::new();
     std::io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line");
    input
}

pub fn get_value(op:&str) -> i32{
loop{
    let value=get_input();
    let value_trim=value.trim();
    match value_trim.parse::<i32>(){
        Ok(val)=>{
            println!("{} value {} is in progress...",op,val);
            return val;
        },
        Err(..)=>{
            println!("The input is not an integer.");
        }
    };

}
}
//main
pub fn cli(){

    println!("----------------------------------<<<<Trees, Trees, and More Trees>>>>----------------------------------");
    println!("Trees menu\n----------------\n1-AVL Tree(avl)\n2-Red-Black Tree(rbt)\n----------------\n");
    println!("Operation menu\n----------------\n1-insert \n2-delete \n3-height \n4-count\n5-empty \n6-search \n7-print \n8-print in order \n9-min \n10-max \n11-length \n----------------");
    println!("----------------------------Let's get start!----------------------------");
    loop{
        println!("-----------------------------------------------------------------------------------------");
        println!("Now input a tree type to start. \n(-Q to exit.-help to get more help about the commands.)");
        println!("-AVL\n-RBT\n");
        println!("Your choice:");
        let select_tree=get_input();
        match select_tree.to_lowercase().trim(){
            "avl"=>{
                avl_cli();
            },
            "rbt"=>{
                rbt_cli();
            },
            "help"=>{
                println!("Commands help:\n----------------\n");
                println!("-avl  - Get start with AVL Tree.");
                println!("-rbt  - Get start with Red Black Tree.");
                println!("-Q - Exit the program.");
                println!("-help - Show more commands.\n");
              //  println!(":After inputting operation, press Enter, then input the value.");
            },
            "q"=>break,
            _=>{
                println!("Commands not match, try again.\n");
            }
        }
    }
}

fn avl_cli(){
    println!("::<<<<<AVL Tree>>>>>::");
    let mut tree=AVLTree::<i32>::new();
    show_operations();
    loop{
        println!("your operation:\n(-Q to exit,-help for all operations)");
        let operation=get_input();
        match operation.to_lowercase().trim() {
            "insert"=>{
                let val=get_value("insert");
                let exist = tree.search(val);
                if exist ==false {
                    tree.insert(val);
                    println!("done!");
                }else{
                    println!("The value is already in the tree! please choose another value.");
                }
            }
            "delete"=>{
                let val=get_value("delete");
                let exist = tree.search(val);
                if exist==true {
                    tree.delete(val);
                    println!("done!");
                }
                else{
                    println!("Cannot delete a value not exist in the tree! Try again!");
                }
        }
            "count"=>{
                println!("The number of leaves of the current avl tree is:{}",tree.count_leaves());
            }
            "height"=>{
                println!("The height of the current avl tree is:{}",tree.height());
            }
            "empty"=>{
                println!("Q:Is this an empty tree? A: {:?}",tree.is_empty());
            }
            "search"=>{
                let val=get_value("search");
              println!("Q:{} exist in tree? A: {:?}",val,tree.search(val));
            }
            "print"=>{
                println!("The in-order traverse of your tree is:");
                tree.print_in_order();
                println!("\n");
            }
            "printall"=>{
                println!("Your current avl tree is:{:#?}",tree);
            }
            "max" =>{
                if let Some(max)=tree.get_max(){
                    println!("The max value in your tree is:{}",max);
                }else { println!("There is no value in your tree! Please do insert operation first.") ;}
            }
            "min"=>{
                if let Some(min)=tree.get_min(){
                    println!("The min value in your tree is:{}",min);
                }else { println!("There is no value in your tree! Please do insert operation first.") ;}
            }
            "length"=>{
                println!("The length of your tree is:{}",tree.len());
            }
            "q"=>return,
            "help"=>show_operations(),
            _=>println!("Commands not match,try again"),
            }
    }


}
fn rbt_cli() {
    println!("::<<<<<Red Black Tree>>>>>::");
    let mut tree = RBTree::<i32>::new();
    show_operations();
    loop {
        println!("your operation:\n(-Q to exit,-help for all operations)");
        let operation = get_input();
        match operation.to_lowercase().trim() {
            "insert" => {
                let val = get_value("insert");
                let exist = tree.search(val);
                if exist ==false {
                    tree.insert(val);
                    println!("done!");
                }else{
                    println!("The value is already in the tree! please choose another value.");
                }

            }
            "delete" => {
                let val = get_value("delete");
                let exist = tree.search(val);
                if exist == true {
                    tree.delete(val);
                    println!("done!");
                } else {
                    println!("Cannot delete a value not exist in the tree! Try again!");
                }
            }
            "count" => {
                println!("The number of leaves of the current red-black tree is:{}", tree.count_leaves());
            }
            "height" => {
                println!("The height of the current red-black tree is:{}", tree.height());
            }
            "empty" => {
                println!("Q:Is this an empty tree? A: {:?}", tree.is_empty());
            }
            "search" => {
                let val = get_value("search");
                println!("Q:{} exist in tree? A: {:?}", val, tree.search(val));
            }
            "print" => {
                println!("The in-order traverse of your tree is:");
                tree.print_in_order();
                println!("\n");
            }
            "printall"=>{
                println!("Your current red-black tree is:{:#?}",tree);
            }
            "max" =>{
               if let Some(max)=tree.get_max(){
                   println!("The max value in your tree is:{}",max);
               }else { println!("There is no value in your tree! Please do insert operation first.") ;}
            }
            "min"=>{
                if let Some(min)=tree.get_min(){
                    println!("The min value in your tree is:{}",min);
                }else { println!("There is no value in your tree! Please do insert operation first.") ;}
            }
            "length"=>{
                println!("The length of your tree is:{}",tree.len());
            }
            "q" => return,
            "help" => show_operations(),
            _ => println!("Commands are not match,try again"),
        }
    }
}

fn show_operations(){
    println!("\nOperation menu on trees: \n---------------");
    println!("\n(note: 1.Input the operation 2.press enter then input the value.)\n---------------");
    println!("-insert   -insert a node into the tree.");
    println!("-delete    -delete a node from the tree.");
    println!("-count    -count the number of leaves of the tree.");
    println!("-height   -return the height of the tree");
    println!("-empty    -check if the tree is empty");
    println!("-search   - search from the tree).");
    println!("-print    -print the in-order traversal of the tree ");
    println!("-printall    -print the tree ");
    println!("-max     - get the maximum value in the tree");
    println!("-min     - get the get_minimum value in the tree");
    println!("-length     - get the length of the tree");
    println!("-Q     - exit and delete the current tree \n");
}
