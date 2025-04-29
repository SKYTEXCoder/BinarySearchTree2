use crate::structure::bst::BstNodeLink;
use crate::structure::tree::NodeLink;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufWriter, Write};

fn node_id(node: &BstNodeLink) -> usize {
    node.as_ptr() as usize
}

fn write_graphviz_dot_notation<W: Write>(root: &BstNodeLink, mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "digraph BinaryTree {{")?;
    let mut queue: VecDeque<std::rc::Rc<std::cell::RefCell<crate::structure::bst::BstNode>>> =
        VecDeque::new();
    queue.push_back(root.clone());
    while let Some(node_link) = queue.pop_front() {
        let node: std::cell::Ref<'_, crate::structure::bst::BstNode> = node_link.borrow();
        let this_id: usize = node_id(&node_link);
        writeln!(writer, "    {} [label=\"{}\"];", this_id, node.key.unwrap())?;
        if let Some(ref left) = node.left {
            let left_id: usize = node_id(left);
            writeln!(
                writer,
                "    {} -> {} [label=\"left\", style=solid, color=red];",
                this_id, left_id
            )?;
            queue.push_back(left.clone());
        }
        if let Some(ref right) = node.right {
            let right_id: usize = node_id(right);
            writeln!(
                writer,
                "    {} -> {} [label=\"right\", style=solid, color=green];",
                this_id, right_id
            )?;
            queue.push_back(right.clone());
        }
        if let Some(ref parent_weak) = node.parent {
            if let Some(parent_rc) = parent_weak.upgrade() {
                let parent_id: usize = node_id(&parent_rc);
                writeln!(
                    writer,
                    "    {} -> {} [label=\"parent\", style=solid, color=blue];",
                    this_id, parent_id
                )?;
            }
        }
    }
    writeln!(writer, "}}")?;
    Ok(())
}

pub fn generate_dotfile_bst_better(root: &BstNodeLink, output_path: &str) {
    let file: File = File::create(output_path).expect("Unable to create .dot file");
    let writer: BufWriter<File> = BufWriter::new(file);
    write_graphviz_dot_notation(root, writer).expect("Unable to write to .dot file");
}

pub fn print_graph(root: &BstNodeLink) {
    write_graphviz_dot_notation(root, std::io::stdout()).expect("Unable to print dot graph");
}

/**
 * @root: root node of the tree in NodeLink Type
 * @output_path: write the graphviz structure to output_path
 * Generate graphviz dot file given a NodeLink, you will traverse from root to all leaves incrementally,
 * as you proceed wrote the progress to dot file
 */
pub fn generate_dotfile(root: &NodeLink, output_path: &str) {
    let graph_name = " tree";
    let preamble: String = "graph".to_owned() + graph_name + "{\n";
    let epilogue = "}";
    //pass either left child or right child but not the root
    let graph_arrangement = node_traversal(root);
    //traverse the node as usual
    let final_text: String = preamble + &graph_arrangement + epilogue;
    let mut output: File = File::create(output_path).expect("Failed to create");
    let _ = output.write_all(final_text.as_bytes());
}

/**
 * We will print string as we traverse, node by node
 * at most a line per node printing, e.g: a--b;
 * traversal mode in BFS
 */
fn node_traversal(node: &NodeLink) -> String {
    let mut new_info: String = "".to_string();
    //we print the child nodes first
    let left_child: &Option<std::rc::Rc<std::cell::RefCell<crate::structure::tree::Node>>> =
        &node.borrow().left;
    //won't print anything if left child is None
    new_info += &print_child(&node, left_child.as_ref());
    let right_child: &Option<std::rc::Rc<std::cell::RefCell<crate::structure::tree::Node>>> =
        &node.borrow().right;
    new_info += &print_child(&node, right_child.as_ref());
    //now we need to traverse deeper
    if left_child.is_some() {
        new_info += &node_traversal(&left_child.as_ref().unwrap());
    }
    if right_child.is_some() {
        new_info += &node_traversal(&right_child.as_ref().unwrap());
    }
    return new_info;
}

fn print_child(parent_node: &NodeLink, child_node: Option<&NodeLink>) -> String {
    let mut new_info: String = "".to_string();
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

pub fn generate_dotfile_bst(root: &BstNodeLink, output_path: &str) {
    let graph_name = " tree";
    let preamble: String = "graph".to_owned() + graph_name + "{\n";
    let epilogue = "}";
    //pass either left child or right child but not the root
    let graph_arrangement: String = node_traversal_bst(root);
    //traverse the node as usual
    let final_text: String = preamble + &graph_arrangement + epilogue;
    let mut output: File = File::create(output_path).expect("Failed to create");
    let _ = output.write_all(final_text.as_bytes());
}

fn node_traversal_bst(node: &BstNodeLink) -> String {
    let mut new_info: String = "".to_string();
    //we print the child nodes first
    let left_child: &Option<std::rc::Rc<std::cell::RefCell<crate::structure::bst::BstNode>>> =
        &node.borrow().left;
    //won't print anything if left child is None
    new_info += &print_child_bst(&node, left_child.as_ref());
    let right_child: &Option<std::rc::Rc<std::cell::RefCell<crate::structure::bst::BstNode>>> =
        &node.borrow().right;
    new_info += &print_child_bst(&node, right_child.as_ref());
    //now we need to traverse deeper
    if left_child.is_some() {
        new_info += &node_traversal_bst(&left_child.as_ref().unwrap());
    }
    if right_child.is_some() {
        new_info += &node_traversal_bst(&right_child.as_ref().unwrap());
    }
    return new_info;
}

fn print_child_bst(parent_node: &BstNodeLink, child_node: Option<&BstNodeLink>) -> String {
    let mut new_info: String = "".to_string();
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
