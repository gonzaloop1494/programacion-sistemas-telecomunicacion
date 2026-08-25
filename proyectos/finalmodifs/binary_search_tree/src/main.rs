pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}
pub struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}


use std::fmt::Display;


use BinaryTree::*;


impl<T: Ord + Clone + Display> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        Empty
    }


    pub fn peek_all_accum(&self, accum: &mut Vec<T>, ascending: bool) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                if ascending {
                    node.left.peek_all_accum(accum, ascending);
                    accum.push(node.value.clone());
                    node.right.peek_all_accum(accum, ascending);
                } else {
                    node.right.peek_all_accum(accum, ascending);
                    accum.push(node.value.clone());
                    node.left.peek_all_accum(accum, ascending);
                }
            }
        }
    }


    pub fn peek_all(&self, ascending: bool) -> Vec<T> {
        let mut accum: Vec<T> = Vec::new();
        self.peek_all_accum(&mut accum, ascending);
        accum
    }


    pub fn insert(&mut self, new_value: T) {
        match self {
            Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    value: new_value,
                    left: Empty,
                    right: Empty,
                }))
            }
            NonEmpty(ref mut node) => {
                if new_value <= node.value {
                    node.left.insert(new_value);
                } else {
                    node.right.insert(new_value);
                }
            }
        }
    }


    pub fn exists(&self, value: T) -> bool {
        match self {
            Empty => {
                false
            }
            NonEmpty(ref node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    node.left.exists(value)
                } else {
                    node.right.exists(value)
                }
            }
        }
    }


    fn number_of_nodes(&self) -> usize {
        match self {
            Empty => {
                0
            }
            NonEmpty(ref node) => {
                1 + node.left.number_of_nodes() + node.right.number_of_nodes()
            }
        }
    }


    fn depth(&self) -> usize {
        match self {
            Empty => {
                0
            }
            NonEmpty(ref node) => {
                1 + node.left.depth().max(node.right.depth())
            }
        }
    }


    fn number_of_branches_v1(&self) -> usize {
        if let Empty = self {
            0
        } else {
            self.number_of_nodes() - 1
        }
    }


    fn number_of_branches_v2(&self) -> usize {
        match self {
            Empty => {
                0
            }
            NonEmpty(ref node) => {
                let mut result: usize = 0;
                if let NonEmpty(ref _left_son) = &node.left {
                    result += 1;
                }
                if let NonEmpty(ref _right_son) = &node.right {
                    result += 1;
                }
                result + node.left.number_of_branches_v2() + node.right.number_of_branches_v2()
            }
        }
    }


    fn number_of_leaves(&self) -> usize {
        match self {
            Empty => {
                0
            }
            NonEmpty(ref node) => {
                match (&node.left, &node.right) {
                    (Empty, Empty) => {
                        1
                    }
                    _ => {
                        node.left.number_of_leaves() + node.right.number_of_leaves()
                    }
                }
            }
        }
    }


    fn leaves(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        self.leaves_aux(&mut result);
        result
    }


    fn leaves_aux(&self, result: &mut Vec<T>) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                match (&node.left, &node.right) {
                    (Empty, Empty) => {
                        result.push(node.value.clone());
                    }
                    _ => {
                        node.left.leaves_aux(result);
                        node.right.leaves_aux(result);
                    }
                }
            }
        }
    }


    fn min_r(&self) -> Option<T> {
        match self {
            Empty => {
                None
            }
            NonEmpty(ref node) => {
                if let Empty = &node.left {
                    Some(node.value.clone())
                } else {
                    node.left.min_r()
                }
            }
        }
    }


    fn min_nr(&self) -> Option<T> {
        let mut result: Option<T> = None;
        let mut current = self;
        while let NonEmpty(ref node) = current {
            result = Some(node.value.clone());
            current = &node.left;
        }
        result
    }


    fn max_r(&self) -> Option<T> {
        match self {
            Empty => {
                None
            }
            NonEmpty(ref node) => {
                if let Empty = &node.right {
                    Some(node.value.clone())
                } else {
                    node.right.max_r()
                }
            }
        }
    }


    fn max_nr(&self) -> Option<T> {
        let mut result: Option<T> = None;
        let mut current = self;
        while let NonEmpty(ref node) = current {
            result = Some(node.value.clone());
            current = &node.right;
        }
        result
    }


    fn print_preorder(&self) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                println!("{}", node.value);
                node.left.print_preorder();
                node.right.print_preorder();
            }
        }
    }


    fn preorder(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        self.preorder_aux(&mut result);
        result
    }


    fn preorder_aux(&self, result: &mut Vec<T>) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                result.push(node.value.clone());
                node.left.preorder_aux(result);
                node.right.preorder_aux(result);
            }
        }
    }


    fn print_inorder(&self) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                node.left.print_inorder();
                println!("{}", node.value);
                node.right.print_inorder();
            }
        }
    }


    fn inorder(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        self.inorder_aux(&mut result);
        result
    }


    fn inorder_aux(&self, result: &mut Vec<T>) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                node.left.inorder_aux(result);
                result.push(node.value.clone());
                node.right.inorder_aux(result);
            }
        }
    }


    fn print_posorder(&self) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                node.left.print_posorder();
                node.right.print_posorder();
                println!("{}", node.value);
            }
        }
    }


    fn posorder(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        self.posorder_aux(&mut result);
        result
    }


    fn posorder_aux(&self, result: &mut Vec<T>) {
        match self {
            Empty => {}
            NonEmpty(ref node) => {
                node.left.posorder_aux(result);
                node.right.posorder_aux(result);
                result.push(node.value.clone());
            }
        }
    }
}


fn main() {
    let mut tree = BinaryTree::new();


    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(2);
    tree.insert(7);
    tree.insert(12);
    tree.insert(17);


    println!("Tree in ascending order: {:?}", tree.peek_all(true));
    println!("Tree in descending order: {:?}", tree.peek_all(false));


    println!("Does 7 exist?: {}", tree.exists(7));
    println!("Does 12 exist?: {}", tree.exists(12));
    println!("Does 20 exist?: {}", tree.exists(20));


    println!("Number of nodes: {}", tree.number_of_nodes());


    println!("Depth of tree: {}", tree.depth());


    println!("Number of branches (v1): {}", tree.number_of_branches_v1());
    println!("Number of branches (v2): {}", tree.number_of_branches_v2());


    println!("Number of leaves: {}", tree.number_of_leaves());
    println!("Leaves: {:?}", tree.leaves());


    println!("Minimum R value in tree: {:?}", tree.min_r());
    println!("Minimum NR value in tree: {:?}", tree.min_nr());
    println!("Maximum R value in tree: {:?}", tree.max_r());
    println!("Maximum NR value in tree: {:?}", tree.max_nr());


    tree.print_preorder();
    println!("Preorder: {:?}", tree.preorder());
    tree.print_inorder();
    println!("Inorder: {:?}", tree.inorder());
    tree.print_posorder();
    println!("Posorder: {:?}", tree.posorder());


    let empty_tree: BinaryTree<i32> = BinaryTree::new();
    println!("Empty tree number of nodes: {}", empty_tree.number_of_nodes());
    println!("Empty tree depth: {}", empty_tree.depth());
    println!("Empty tree min R: {:?}", empty_tree.min_r());
    println!("Empty tree min NR: {:?}", empty_tree.min_nr());
    println!("Empty tree max R: {:?}", empty_tree.max_r());
    println!("Empty tree max NR: {:?}", empty_tree.max_nr());
    println!("Empty tree number of leaves: {}", empty_tree.number_of_leaves());
    println!("Empty tree leaves: {:?}", empty_tree.leaves());
    empty_tree.print_preorder();
    println!("Empty tree Preorder: {:?}", empty_tree.preorder());
    empty_tree.print_inorder();
    println!("Empty tree Inorder: {:?}", empty_tree.inorder());
    empty_tree.print_posorder();
    println!("Empty tree Posorder: {:?}", empty_tree.posorder());
}

