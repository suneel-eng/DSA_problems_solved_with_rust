#[derive(Debug)]
pub struct TreeNode {
    pub data: u8,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>
}

pub fn inorder_traversal(tree_root: Option<Box<TreeNode>>) -> Vec<u8> {
    // In-order traversal is a way to visit all the nodes in a binary tree,
    // which is a type of data structure where each node can have at most two children.

    // Here's how it works:

    // First, we visit the left subtree recursively.
    // This means we go to the left child of a node and repeat the process
    // until we reach a node that doesn't have a left child.

    // Then, we visit the current node.

    // Finally, we visit the right subtree recursively. Similar to the left subtree,
    // we go to the right child of a node and repeat the process
    // until we reach a node that doesn't have a right child.

    // So, in simpler terms, in-order traversal means visiting nodes in a binary tree
    // in the order of left child, current node, right child.
    // This method is called "in-order" because we visit nodes in the order that respects
    // the left-to-right ordering of the values in the tree.

    fn travel(node: &Option<Box<TreeNode>>, result: &mut Vec<u8>) {

        if node.is_none() {
            return;
        }

        travel(&node.as_ref().unwrap().left, result);
        result.push(node.as_ref().unwrap().data);
        travel(&node.as_ref().unwrap().right, result);
    }

    let mut items: Vec<u8> = Vec::new();

    travel(&tree_root, &mut items);

    items
}

pub fn preorder_traversal(tree_root: Option<Box<TreeNode>>) -> Vec<u8> {

    // Preorder traversal is another way to visit all the nodes in a binary tree,
    // which is a type of data structure where each node can have at most two children.

    // Here's how it works:

    // We start by visiting the current node.
    // Then, we recursively visit the left subtree,
    // meaning we go to the left child of the current node and repeat the process.

    // After visiting the left subtree, we recursively visit the right subtree,
    // meaning we go to the right child of the current node and repeat the process.
    
    // So, in simpler terms, preorder traversal means visiting nodes
    // in a binary tree in the order of current node, left child, right child.
    // This method is called "preorder" because we visit the current node before visiting its children.

    fn travel(node: &Option<Box<TreeNode>>, result: &mut Vec<u8>) {

        if node.is_none() {
            return;
        }

        result.push(node.as_ref().unwrap().data);
        travel(&node.as_ref().unwrap().left, result);
        travel(&node.as_ref().unwrap().right, result);
    }

    let mut items: Vec<u8> = Vec::new();

    travel(&tree_root, &mut items);

    items
}

pub fn postorder_traversal(tree_root: Option<Box<TreeNode>>) -> Vec<u8> {

    // Postorder traversal is yet another way to visit all the nodes in a binary tree,
    // which is a type of data structure where each node can have at most two children.

    // Here's how it works:

    // We start by recursively visiting the left subtree,
    // meaning we go to the left child of the current node and repeat the process.

    // Then, we recursively visit the right subtree, meaning we go to
    // the right child of the current node and repeat the process.
    
    // After visiting both subtrees, we finally visit the current node.
    
    //  So, in simpler terms, postorder traversal means visiting nodes in a binary tree
    // in the order of left child, right child, current node.
    // This method is called "postorder" because we visit the current node after visiting its children.
    
    fn travel(node: &Option<Box<TreeNode>>, result: &mut Vec<u8>) {

        if node.is_none() {
            return;
        }

        travel(&node.as_ref().unwrap().left, result);
        travel(&node.as_ref().unwrap().right, result);
        result.push(node.as_ref().unwrap().data);
    }

    let mut items: Vec<u8> = Vec::new();

    travel(&tree_root, &mut items);

    items
}

pub fn size_of_tree(tree_root: Option<Box<TreeNode>>) -> u8 {

    // size of tree means number of nodes the tree has.

    fn travel(node: &Option<Box<TreeNode>>, result: &mut u8) {

        if node.is_none() {
            return;
        }

        travel(&node.as_ref().unwrap().left, result);
        travel(&node.as_ref().unwrap().right, result);
        *result += 1;
    }

    let mut node_count: u8 = 0;

    travel(&tree_root, &mut node_count);

    node_count
}

pub fn sum_of_nodes(tree_root: Option<Box<TreeNode>>) -> u64 {

    // size of tree means number of nodes the tree has.

    fn travel(node: &Option<Box<TreeNode>>, result: &mut u64) {

        if node.is_none() {
            return;
        }

        travel(&node.as_ref().unwrap().left, result);
        travel(&node.as_ref().unwrap().right, result);
        *result += node.as_ref().unwrap().data as u64;
    }

    let mut nodes_sum: u64 = 0;

    travel(&tree_root, &mut nodes_sum);

    nodes_sum
}

pub fn height_of_tree(tree_root: Option<Box<TreeNode>>) -> i64 {

    fn travel(node: &Option<Box<TreeNode>>) -> i64 {

        if node.is_none() {
            return -1;
        }

        let left_height: i64 = travel(&node.as_ref().unwrap().left);
        let right_height: i64 = travel(&node.as_ref().unwrap().right);
        return std::cmp::max(left_height, right_height) + 1;
    }

    let height: i64 = travel(&tree_root);

    height
}