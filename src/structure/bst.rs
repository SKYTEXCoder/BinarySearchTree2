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

    fn new_with_rc_pointer(key: i32) -> BstNodeLink {
        Rc::new(RefCell::new(BstNode::new(key)))
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
        // currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
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

    fn clone_weak_node(weak_node: &WeakBstNodeLink) -> WeakBstNodeLink {
        Weak::clone(weak_node)
    }

    fn clone_optional_weak_bst_node(
        optional_weak_bst_node: &Option<WeakBstNodeLink>,
    ) -> Option<WeakBstNodeLink> {
        optional_weak_bst_node.as_ref().map(Self::clone_weak_node)
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

    fn downgrade_strong_to_weak(optional_node: Option<BstNodeLink>) -> Option<WeakBstNodeLink> {
        match optional_node {
            None => None,
            Some(x) => Some(Rc::downgrade(&x)),
        }
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
            Some(k) if *value > k => {
                if let Some(ref right) = self.right {
                    right.borrow().tree_search(value)
                } else {
                    None
                }
            }
            Some(_) => None,
            None => None,
        }
    }

    pub fn tree_search_correct(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self
                    .left
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .tree_search_correct(value);
            } else if self.right.is_some() {
                return self
                    .right
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .tree_search_correct(value);
            }
        }
        None
    }

    /**seek minimum by recursion
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

    pub fn minimum_correct(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum_correct();
            }
        }
        self.get_bst_nodelink_copy()
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

    pub fn maximum_correct(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum_correct();
            }
        }
        self.get_bst_nodelink_copy()
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

    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> BstNodeLink {
        // create a shadow of x_node so it can mutate
        let mut x_node: &Rc<RefCell<BstNode>> = x_node;
        let right_node: &Option<Rc<RefCell<BstNode>>> = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node) != true {
            return right_node.clone().unwrap().borrow().minimum();
        }
        let mut y_node: Option<Rc<RefCell<BstNode>>> =
            BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right: &Option<Rc<RefCell<BstNode>>> =
            &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node)
            && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone())
        {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent: Weak<RefCell<BstNode>> =
                y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }
        // in case our successor traversal yields root, that means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return x_node.clone();
        }
        // defaultly return either self or x_node
        y_node.clone().unwrap()
    }

    /// Inserts a new node with the given key into the BST.
    ///
    /// # Arguments
    ///
    /// * `bst_node_link` - Reference to the root node of the BST.
    /// * `key` - The key value to insert.
    ///
    /// # Returns
    ///
    /// * `Some(BstNodeLink)` if the insertion is successful.
    /// * `None` if a node with the same key already exists.
    pub fn tree_insert(bst_node_link: &BstNodeLink, key: &i32) -> Option<BstNodeLink> {
        if bst_node_link.borrow().tree_search_correct(&key).is_some() {
            return None;
        }
        let z: BstNodeLink = BstNode::new_bst_nodelink(*key);
        z.borrow_mut().parent = None;
        z.borrow_mut().left = None;
        z.borrow_mut().right = None;
        let mut y: Option<BstNodeLink> = None;
        let mut x: Option<BstNodeLink> = Some(bst_node_link.clone());
        while let Some(x_rc_pointer) = x {
            y = Some(x_rc_pointer.clone());
            if z.borrow().key < x_rc_pointer.borrow().key {
                x = x_rc_pointer.borrow().left.clone();
            } else {
                x = x_rc_pointer.borrow().right.clone();
            }
        }
        if let Some(ref y_rc_pointer) = y {
            if z.borrow().key < y_rc_pointer.borrow().key {
                y_rc_pointer.borrow_mut().left = Some(z.clone());
            } else {
                y_rc_pointer.borrow_mut().right = Some(z.clone());
            }
            z.borrow_mut().parent = Some(Rc::downgrade(y_rc_pointer));
        } else {
            z.borrow_mut().parent = None;
            return Some(z.clone());
        }
        Some(z)
    }

    /// Replaces one subtree as a child of its parent with another subtree.
    ///
    /// # Arguments
    ///
    /// * `u` - The node to be replaced.
    /// * `v` - The node to replace `u` with (can be `None`).
    ///
    /// # Returns
    ///
    /// * `true` if the transplant operation is successful.
    /// * `false` if the parent pointer cannot be upgraded.
    pub fn transplant(&mut self, u: &BstNodeLink, v: &Option<BstNodeLink>) -> bool {
        if let Some(ref u_parent_weak_pointer) = &u.borrow().parent {
            if let Some(u_parent_rc_pointer) = u_parent_weak_pointer.upgrade() {
                let is_left_children: bool =
                    if let Some(ref left_children) = u_parent_rc_pointer.borrow().left {
                        Rc::ptr_eq(&u, left_children)
                    } else {
                        Rc::ptr_eq(&u, &BstNode::new_bst_nodelink(i32::default()))
                    };
                if is_left_children {
                    u_parent_rc_pointer.borrow_mut().left = v.clone();
                    if let Some(ref left) = u_parent_rc_pointer.borrow().left {
                        left.borrow_mut().parent = Some(BstNode::downgrade(&u_parent_rc_pointer));
                    }
                } else {
                    u_parent_rc_pointer.borrow_mut().right = v.clone();
                    if let Some(ref right) = u_parent_rc_pointer.borrow().right {
                        right.borrow_mut().parent = Some(BstNode::downgrade(&u_parent_rc_pointer));
                    }
                }
            } else {
                println!("Cannot perform transplanting operation: Cannot upgrade parent pointer from Weak<RefCell<BstNode>> to Rc<RefCell<BstNode>>.");
                return false;
            }
        } else {
            if let Some(ref v_rc_pointer) = &v {
                v_rc_pointer.borrow_mut().parent = None
            }
        }
        if let Some(ref v_rc_pointer) = &v {
            v_rc_pointer.borrow_mut().parent =
                BstNode::clone_optional_weak_bst_node(&u.borrow().parent);
            if let Some(ref left) = v_rc_pointer.borrow().left {
                left.borrow_mut().parent = Some(BstNode::downgrade(v_rc_pointer));
            }
            if let Some(ref right) = v_rc_pointer.borrow().right {
                right.borrow_mut().parent = Some(BstNode::downgrade(v_rc_pointer));
            }
        }
        return true;
    }

    /// Deletes the specified node from the BST.
    ///
    /// # Arguments
    ///
    /// * `z` - The node to delete.
    ///
    /// # Returns
    ///
    /// * `true` if the deletion is successful.
    pub fn tree_delete(&mut self, z: &BstNodeLink) -> bool {
        if z.borrow().left.is_none() {
            self.transplant(&z.clone(), &z.borrow().right.clone());
        } else if z.borrow().right.is_none() {
            self.transplant(&z.clone(), &z.borrow().left.clone());
        } else {
            let successor: Rc<RefCell<BstNode>> =
                z.borrow().right.as_ref().unwrap().borrow().minimum();
            if !Rc::ptr_eq(&successor, z.borrow().right.as_ref().unwrap()) {
                self.transplant(&successor.clone(), &successor.borrow().right.clone());
                successor.borrow_mut().right = z.borrow().right.clone();
                if let Some(ref right) = successor.borrow().right {
                    right.borrow_mut().parent = Some(Rc::downgrade(&successor));
                }
            }
            self.transplant(&z.clone(), &Some(successor.clone()));
            successor.borrow_mut().left = z.borrow().left.clone();
            if let Some(ref left) = successor.borrow().left {
                left.borrow_mut().parent = Some(Rc::downgrade(&successor));
            };
        }
        return true;
    }

    /// Deletes a node with the specified key from the BST.
    ///
    /// # Arguments
    ///
    /// * `key` - The key value of the node to delete.
    ///
    /// # Returns
    ///
    /// * `true` if the node is found and deleted.
    /// * `false` if the node with the given key does not exist.
    pub fn tree_delete_with_key(&mut self, key: i32) -> bool {
        let node_to_delete: Option<BstNodeLink> = self.tree_search_correct(&key);
        if let Some(node) = node_to_delete {
            self.tree_delete(&node);
            return true;
        } else {
            println!(
                "A node with the i32-typed key value of {} is not found inside this tree or subtree.",
                key
            );
            return false;
        }
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
     * private function that returns the boolean value of true if the given node doesn't have a designated parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}
