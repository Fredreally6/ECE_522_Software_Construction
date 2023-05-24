pub use treescrate::preclude::{AVLTree,BaseTree};
//TB rewrite
pub fn main() {
        println!("Demo for AVLTree, start initializing...:");
        let mut tree = AVLTree::new();
        tree.insert(3);
        println!("insert 3...done!");
        tree.insert(4);
        println!("insert 4...done!");
        tree.insert(8);
        println!("insert 8...done!");
       /* tree.insert(1);
        println!("insert 1...done!");
        tree.insert(5);
        println!("insert 5...done!");*/
        println!("----------------------<<<<Demo for AVLTree>>>>----------------------");
        print!("-The tree ");
        tree.print_in_order();
        println!("\n");
        println!("-height: {}", tree.height());
        println!("-empty?: {}", tree.is_empty());
        println!("-leaves number: {}", tree.count_leaves());
        println!("-contains 8?: {}", tree.search(8));
        println!("-contains 1?: {}", tree.search(1));
        println!("-min value in the tree: {}", tree.get_min().unwrap());
        println!("-max value in the tree: {}", tree.get_max().unwrap());
        println!("-length of the tree: {}", tree.len());
        println!("-print the AVL tree:{:#?}",tree);
        println!("\n");
    }
