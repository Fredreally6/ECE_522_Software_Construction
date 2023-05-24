use criterion::{black_box, criterion_group, criterion_main, Criterion};
use treescrate::avltree::AVLTree;
use treescrate::rbtree::RBTree;
use treescrate::bstree::{BSRootTree, BSTree, Node};
use treescrate::preclude::BaseTree;

//benchmark for avl
fn avl_insert_search_bench(tree_size:i32) {
    let mut tree = AVLTree::new();
    for i in 1..tree_size {
        tree.insert(i);
    }
    for j in 1..tree_size / 10 {
        tree.search(j);
    }
}
    //benchmark for RBtree
fn rb_insert_search_bench(tree_size:i32){
    let mut tree=RBTree::new();
    for i in 1..tree_size{
        tree.insert(i);
    }
    for j in 1..tree_size/10{
        tree.search(j);
    }
}
    //benchmark for BStree
fn bs_insert_search_bench(tree_size:i32){
    let mut tree=BSRootTree::new();
    for i in 1..tree_size{
        tree.insert(i);
    }
    for j in 1..tree_size/10{
        tree.search(j);
    }
}


    fn benchmark_test(c:&mut Criterion){
     //  let tree_size= vec![10000,40000,70000,100000,130000];
        let tree_size= vec![1000,5000,10000,20000,40000,70000,90000,100000,130000];
        for i in tree_size.iter(){
            c.bench_function("rb_bench", |b| b.iter(|| rb_insert_search_bench(black_box(*i))));
            c.bench_function("avl_bench",|b|b.iter(||avl_insert_search_bench(black_box(*i))));
            c.bench_function("bs_bench",|b|b.iter(||bs_insert_search_bench(black_box(*i))));
        }
    }

criterion_group!(benches, benchmark_test);
criterion_main!(benches);