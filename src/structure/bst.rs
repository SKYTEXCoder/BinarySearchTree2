use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode: BstNode = BstNode::new(value);
        let currentlink: Rc<RefCell<BstNode>> = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode: BstNode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink: Rc<RefCell<BstNode>> = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    fn clone_node(node: &BstNodeLink) -> BstNodeLink {
        Rc::clone(node)
    }

    fn clone_optional_node(optional_node: &Option<BstNodeLink>) -> Option<BstNodeLink> {
        optional_node.as_ref().map(Self::clone_node)
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node: Rc<RefCell<BstNode>> = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node: Rc<RefCell<BstNode>> = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        match self.key {
            Some(k) if k == *value => Some(self.get_bst_nodelink_copy()),
            Some(k) if *value < k => {
                if let Some(ref left) = self.left {
                    left.borrow().tree_search(value)
                } else {
                    None
                }
            }
            Some(_) => {
                if let Some(ref right) = self.right {
                    right.borrow().tree_search(value)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        let mut current = self.get_bst_nodelink_copy();
        loop {
            let left = current.borrow().left.clone();
            if let Some(left_node) = left {
                current = left_node;
            } else {
                break;
            }
        }
        current
    }

    pub fn maximum(&self) -> BstNodeLink {
        let mut current = self.get_bst_nodelink_copy();
        loop {
            let right = current.borrow().right.clone();
            if let Some(right_node) = right {
                current = right_node;
            } else {
                break;
            }
        }
        current
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent: Option<Rc<RefCell<BstNode>>> =
            BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * Find node successor according to the book
     * Possible to return self, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> BstNodeLink {
        let x_borrow = x_node.borrow();
        if let Some(ref right) = x_borrow.right {
            return right.borrow().minimum();
        }
        let mut current = x_node.clone();
        let mut optional_parent = x_borrow.parent.clone().and_then(|w| w.upgrade());
        drop(x_borrow);
        while let Some(parent_rc_pointer) = optional_parent {
            let parent = parent_rc_pointer.borrow();
            if let Some(ref left) = parent.left {
                if Rc::ptr_eq(left, &current) {
                    return parent_rc_pointer.clone();
                }
            }
            current = parent_rc_pointer.clone();
            optional_parent = parent.parent.clone().and_then(|w| w.upgrade());
        }
        return x_node.clone();
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
