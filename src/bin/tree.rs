use std::cell::RefCell;
use std::rc::{Rc, Weak};
/*
Hard Exercise: Building a Tree Data Structure
Let's create a more complex exercise to solidify your understanding of Rc. You will implement a tree data structure where each node can have multiple children, and each child can have a reference to its parent using Rc.

Steps
Define the Tree Node: Define a TreeNode struct with Rc for parent and children.
Implement Methods:
Add a child to a node.
Display the tree.
Test the Tree: Create a tree, add children, and display the structure.
 */

struct TreeNode {
    value: u32,
    parent: RefCell<Weak<TreeNode>>,
    children: RefCell<Vec<Rc<TreeNode>>>
}

impl TreeNode {

    fn new(init_value: u32) -> Self {
        TreeNode {
            value: init_value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        }
    }
    fn add_child(parent: &Rc<TreeNode>, child: Rc<TreeNode>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }

    fn remove_child(parent: &Rc<TreeNode>, child_value: u32) {
        let child_pos = parent.children
            .borrow_mut()
            .iter()
            .position(|child| child.value == child_value)
            .unwrap();

        let removed_child = parent.children.borrow_mut().remove(child_pos);
        removed_child.parent.take();
    }

    fn display_tree(node: &Rc<TreeNode>, depth: usize) {
        for _ in 0..depth {
            print!(" ")
        }

        println!("value: {}", node.value);

        for child in node.children.borrow().iter() {
            Self::display_tree(child, depth + 1);
        }

    }

    fn calculate_depth(node: &Rc<TreeNode>) -> usize {
        if node.children.borrow().is_empty() {
            0
        } else {
            1 + node.children.borrow().iter().map(|child| Self::calculate_depth(child) ).max().unwrap()
        }
    }
}

fn main() {
    let root = Rc::new(TreeNode::new(1));

    let child1 = Rc::new(TreeNode::new(2));
    let child2 = Rc::new(TreeNode::new(3));
    let child3 = Rc::new(TreeNode::new(4));

    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());

    TreeNode::add_child(&child1, child3.clone());

    TreeNode::display_tree(&root, 0);

    println!("Depth of the tree: {}", TreeNode::calculate_depth(&root));
}