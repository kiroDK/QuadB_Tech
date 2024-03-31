#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => 1 + max(max_depth(&node.left), max_depth(&node.right)),
        None => 0,
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("This example will use a hardcoded binary tree.");
    let tree = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));

    let depth = max_depth(&tree);
    println!("Maximum depth of the tree is {}", depth);
}
