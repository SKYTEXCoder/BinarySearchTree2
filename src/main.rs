mod structure;
mod tool;

use crate::structure::bst::{BstNode, BstNodeLink};
use crate::structure::tree::{Node, NodeLink};
use crate::tool::{
    generate_dotfile, generate_dotfile_bst, generate_dotfile_bst_better, print_graph,
};

fn main() {
    //turn on to test the old code
    //test_binary_tree();
    test_binary_search_tree();
}

fn test_binary_search_tree() {
    let rootlink: BstNodeLink = BstNode::new_bst_nodelink(15);
    {
        let mut root = rootlink.borrow_mut();
        root.add_left_child(&rootlink, 6);
        root.add_right_child(&rootlink, 18);
    }

    //add right subtree
    if let Some(right_tree) = rootlink.borrow().right.clone() {
        let mut right = right_tree.borrow_mut();
        right.add_left_child(&right_tree, 17);
        right.add_right_child(&right_tree, 20);
    }

    //add left subtree
    if let Some(left_tree) = rootlink.borrow().left.clone() {
        {
            let mut left_mutable_borrow = left_tree.borrow_mut();
            left_mutable_borrow.add_left_child(&left_tree, 3);
            left_mutable_borrow.add_right_child(&left_tree, 7);
        }

        // left-left grandchild
        if let Some(term_left) = left_tree.borrow().left.clone() {
            let mut term = term_left.borrow_mut();
            term.add_left_child(&term_left, 2);
            term.add_right_child(&term_left, 4);
        }

        // left-right subtree of 7
        if let Some(node7) = left_tree.borrow().right.clone() {
            let mut n7 = node7.borrow_mut();
            n7.add_right_child(&node7, 13);

            if let Some(grand) = n7.right.clone() {
                let mut g = grand.borrow_mut();
                g.add_left_child(&grand, 9);
            }
        }
    }

    //print the tree at this time
    let mut main_tree_path = "bst_graph.dot";
    generate_dotfile_bst_better(&rootlink, main_tree_path);
    println!("");
    print_graph(&rootlink);

    //tree search test
    let node_result: Option<std::rc::Rc<std::cell::RefCell<BstNode>>> =
        rootlink.borrow().tree_search_correct(&3);
    println!("tree search result {:?}", node_result);
    //min test
    let min_node: std::rc::Rc<std::cell::RefCell<BstNode>> = rootlink.borrow().minimum();
    println!("minimum result {:?}", min_node);
    //max test
    let max_node: std::rc::Rc<std::cell::RefCell<BstNode>> = rootlink.borrow().maximum();
    println!("maximum result {:?}", max_node);
    //root node get test
    let root_node: std::rc::Rc<std::cell::RefCell<BstNode>> = BstNode::get_root(&max_node);
    println!("");
    println!("root node {:?}", root_node);

    //successor test
    let mut successor_node: std::rc::Rc<std::cell::RefCell<BstNode>> =
        BstNode::tree_successor(&root_node);
    println!("");
    print!("Successor of node 15 is ");
    println!("{:?}", successor_node);

    successor_node = BstNode::tree_successor(&min_node);
    print!("Successor of node 2 is ");
    println!("{:?}", successor_node);

    let mut inserted: Option<std::rc::Rc<std::cell::RefCell<BstNode>>> =
        BstNode::tree_insert(&rootlink, &8);
    println!("");
    println!("Inserted node with key 8: {:?}", inserted);
    let duplicate: Option<std::rc::Rc<std::cell::RefCell<BstNode>>> =
        BstNode::tree_insert(&rootlink, &8);
    println!("Attempt to insert duplicate key 8: {:?}", duplicate);

    inserted = BstNode::tree_insert(&rootlink, &16);
    println!("Inserted node with key 16: {:?}", inserted);

    inserted = BstNode::tree_insert(&rootlink, &14);
    println!("Inserted node with key 14: {:?}", inserted);

    inserted = BstNode::tree_insert(&rootlink, &19);
    println!("Inserted node with key 19: {:?}", inserted);

    inserted = BstNode::tree_insert(&rootlink, &10);
    println!("Inserted node with key 19: {:?}", inserted);

    inserted = BstNode::tree_insert(&rootlink, &11);
    println!("Inserted node with key 19: {:?}", inserted);

    inserted = BstNode::tree_insert(&rootlink, &12);
    println!("Inserted node with key 19: {:?}", inserted);

    main_tree_path = "bst_graph_2.dot";
    generate_dotfile_bst_better(&rootlink, main_tree_path);
    println!("");
    print_graph(&rootlink);

    // --- TEST transplant() ---
    // Transplant node 13 with node 9 (should move 9 up to where 13 was)
    let node_13: std::rc::Rc<std::cell::RefCell<BstNode>> =
        rootlink.borrow().tree_search_correct(&13).unwrap();
    let node_9: std::rc::Rc<std::cell::RefCell<BstNode>> =
        rootlink.borrow().tree_search_correct(&9).unwrap();

    {
        let mut root_mut: std::cell::RefMut<'_, BstNode> = rootlink.borrow_mut();
        let result: bool = root_mut.transplant(&node_13.clone(), &Some(node_9.clone()));
        println!(
            "\nSuccessfully transplanted node 13 with node 9: {}",
            result
        );
    }

    main_tree_path = "bst_graph_after_transplant.dot";
    generate_dotfile_bst_better(&rootlink, main_tree_path);
    print_graph(&rootlink);

    // --- TEST tree_delete() ---
    let node_7: std::rc::Rc<std::cell::RefCell<BstNode>> =
        rootlink.borrow().tree_search_correct(&7).unwrap();
    let delete_result: bool = rootlink.borrow_mut().tree_delete(&node_7);

    println!("\nDelete Node 7: {}", delete_result);

    main_tree_path = "bst_graph_after_delete.dot";
    generate_dotfile_bst_better(&rootlink, main_tree_path);
    print_graph(&rootlink);
}

fn test_binary_tree() {
    //create the nodelink of the root node
    let rootlink: NodeLink = Node::new_nodelink(5);

    //add a new left node value
    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    //add a new right node value
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    //println!("{:?}", rootlink);

    //print the tree at this time
    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);

    //add new child values to the left subtree
    let left_subtree = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 2);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 4);
    }

    //add new child values to the right subtree
    let right_subtree = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 10);
    }

    //print the tree again, now been added with more values
    main_tree_path = "prime_t2.dot";
    generate_dotfile(&rootlink, main_tree_path);

    //Call tree depth function at this time
    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    //Call count_nodes function
    let total_nodes = rootlink.borrow().count_nodes();
    println!("Amount of nodes in current tree: {0}", total_nodes);

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    //TODO
    let subtree_count = Node::count_nodes_by_nodelink(&right_subtree.clone().unwrap(), 0);
    println!("Amount of nodes in current subtree: {0}", subtree_count);

    //Get the sibling of the leftsubtree from parent
    let _left_subtree_sibling = Node::get_sibling(&left_subtree.as_ref().unwrap());
    //println!("sibling of left subtree {:?}", left_subtree_sibling);

    //get the left subtree by value
    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree);
    //get the left subtree by full properties
    let another_left_subtree = rootlink
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!(
        "left subtree seek by full property {:?}",
        another_left_subtree
    );

    //Discard the right subtree from parent
    let rootlink2 = rootlink.borrow().get_nodelink_copy();

    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {0}", flag);

    //print the tree again
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&rootlink2, main_tree_path);

    //Call tree depth function at this time
    //TODO
    let depth_now: i32 = rootlink2.borrow().tree_depth();
    println!("Depth after discard {0}", depth_now);

    //Call count_nodes function
    let count_now: i32 = rootlink2.borrow().count_nodes();
    println!("Count nodes after discard {0}", count_now);

    //print the tree again
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}
