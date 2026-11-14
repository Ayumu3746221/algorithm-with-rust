use std::collections::VecDeque;

struct Node {
    value: i32,
    children: Vec<Node>,
}

fn main() {
    let root = Node {
        value: 1,
        children: vec![
            Node {
                value: 2,
                children: vec![Node {
                    value: 4,
                    children: vec![],
                }],
            },
            Node {
                value: 3,
                children: vec![],
            },
        ],
    };

    let target = 4;
    let found = depth_first_search(&root, target);
    let found_recursion = depth_first_search_recursion(&root, target);
    println!("target {} found: {}", target, found);
    println!("recursion target {} found: {}", target, found_recursion);
}

fn depth_first_search(root: &Node, target: i32) -> bool {
    let mut queue: VecDeque<&Node> = VecDeque::new();
    queue.push_back(root);
    while let Some(node) = queue.pop_back() {
        if node.value == target {
            return true;
        }
        for child in &node.children {
            queue.push_back(child);
        }
    }
    false
}

fn depth_first_search_recursion(root: &Node, target: i32) -> bool {
    if root.value == target {
        return true;
    }
    root.children
        .iter()
        .any(|node| -> bool { depth_first_search_recursion(node, target) })
}
