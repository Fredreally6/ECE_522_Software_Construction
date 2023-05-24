use std::env;
#[allow(unused_imports)]
use std::io;



#[path = "../demo/avl_demo.rs"]
mod avl_demo;
#[path = "../demo/red_black_tree_demo.rs"]
mod red_black_tree_demo;

//have to include here.
mod avltree;
mod rbtree;
mod basetree;
mod bstree;
mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
       1=>{ println!("Welcome to use the Red-Black Tree and AVL Tree application!");
           //use loop to let one restart.
           loop {
                 println!("-Use: to access the Command Line Interface.");
                 println!("-View: to check the example Demo.");
                 println!("Your choice:");
            //get the input
            //match input into different choice
               let mut choice=String::new();
               std::io::stdin()
                   .read_line(&mut choice)
                   .expect("Read failed");
               //println!("Your input is:{}",answer);
               if choice.to_lowercase().contains("use"){
                   //go to the cli to use the system
                   cli::cli();
                  // break;
               } else if choice.to_lowercase().contains("view") {
                   loop {//show demo
                       println!("Please input the demo you want to check. \n-avl -rbt (-Q to go back to the beginning page.)");
                       let mut str = String::new();
                       std::io::stdin()
                           .read_line(&mut str)
                           .expect("Read failed");
                       if str.to_lowercase().contains("avl") {
                           avl_demo::main();
                       } else if str.to_lowercase().contains("rbt") {
                           red_black_tree_demo::main();
                       } else if str.to_lowercase().contains("q") {
                           //return to the loop start
                           break;
                       } else { println!("Error Please restart.") }
                   }
               } else { println!("Error! Please restart.") }
           }
       }
       _=>{eprintln!("Error!App quit.")}
    }
}




