use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use std::fs::File;
use std::io::Write;

/**
 * @root: root node of the tree in NodeLink Type
 * @output_path: write the graphviz structure to output_path
 * Generate graphviz dot file given a NodeLink, you will traverse from root to all leaves incrementally,
 * as you proceed wrote the progress to dot file
 */
pub fn generate_dotfile(root: &NodeLink, output_path: &str){
    let graph_name = " tree";
    let preamble = "graph".to_owned() + graph_name + "{\n";
    let epilogue = "}";
    //pass either left child or right child but not the root
    let graph_arrangement = node_traversal(root);
    //traverse the node as usual
    let final_text = preamble + &graph_arrangement + epilogue;
    let mut output = File::create(output_path).expect("Failed to create");
    let _ = output.write_all(final_text.as_bytes());}

/**
 * We will print string as we traverse, node by node
 * at most a line per node printing, e.g: a--b;
 * traversal mode in BFS
 */
fn node_traversal(node: &NodeLink) -> String{
    let mut new_info: String = "".to_string();
    //we print the child nodes first
    let left_child = &node.borrow().left;
    //won't print anything if left child is None
    new_info += &print_child(&node, left_child.as_ref());
    let right_child = &node.borrow().right;
    new_info += &print_child(&node, right_child.as_ref());
    //now we need to traverse deeper
    if left_child.is_some(){
        new_info += &node_traversal(&left_child.as_ref().unwrap());
    }
    if right_child.is_some(){
        new_info += &node_traversal(&right_child.as_ref().unwrap());
    }
    return new_info;
}

fn print_child(parent_node: &NodeLink, child_node: Option<&NodeLink>) -> String{
    let mut new_info = "".to_string();
    if let Some(child) = child_node {
        //concat parent
        new_info += "\t";
        new_info += &parent_node.borrow().value.to_string();
        //next_info += node.borrow().parent.unwrap().value;
        new_info += "--";
        new_info += &child.borrow().value.to_string();
        new_info += ";\n";
    }
    return new_info;
}

pub fn generate_dotfile_bst(root: &BstNodeLink, output_path: &str){
    let graph_name = " tree";
    let preamble = "graph".to_owned() + graph_name + "{\n";
    let epilogue = "}";
    //pass either left child or right child but not the root
    let graph_arrangement = node_traversal_bst(root);
    //traverse the node as usual
    let final_text = preamble + &graph_arrangement + epilogue;
    let mut output = File::create(output_path).expect("Failed to create");
    let _ = output.write_all(final_text.as_bytes());}

fn node_traversal_bst(node: &BstNodeLink) -> String{
    let mut new_info: String = "".to_string();
    //we print the child nodes first
    let left_child = &node.borrow().left;
    //won't print anything if left child is None
    new_info += &print_child_bst(&node, left_child.as_ref());
    let right_child = &node.borrow().right;
    new_info += &print_child_bst(&node, right_child.as_ref());
    //now we need to traverse deeper
    if left_child.is_some(){
        new_info += &node_traversal_bst(&left_child.as_ref().unwrap());
    }
    if right_child.is_some(){
        new_info += &node_traversal_bst(&right_child.as_ref().unwrap());
    }
    return new_info;
}

fn print_child_bst(parent_node: &BstNodeLink, child_node: Option<&BstNodeLink>) -> String{
    let mut new_info = "".to_string();
    if let Some(child) = child_node {
        //concat parent
        new_info += "\t";
        new_info += &parent_node.borrow().key.unwrap().to_string();
        //next_info += node.borrow().parent.unwrap().value;
        new_info += "--";
        new_info += &child.borrow().key.unwrap().to_string();
        new_info += ";\n";
    }
    return new_info;
}

/*
pub fn graph_dotfile_string(root: &NodeLink) -> String{
    ""
}
*/