use std::collections::VecDeque;
use std::vec::Vec;

#[derive(PartialEq, Clone)]
struct Node {
    value: i32,
    children: Vec<Node>,
}

fn main() {
    // 木を構築
    //          1
    //        / | \
    //       2  3  4
    //      /|     |
    //     5 6     7
    let tree = Node {
        value: 1,
        children: vec![
            Node {
                value: 2,
                children: vec![
                    Node {
                        value: 5,
                        children: vec![],
                    },
                    Node {
                        value: 6,
                        children: vec![],
                    },
                ],
            },
            Node {
                value: 3,
                children: vec![],
            },
            Node {
                value: 4,
                children: vec![Node {
                    value: 7,
                    children: vec![],
                }],
            },
        ],
    };

    for target in [1, 6, 7, 0, 99] {
        println!(
            "target = {} -> {}",
            target,
            breadth_first_search(&tree, target) // 参照渡しなので clone 不要
        );
    }
}

fn breadth_first_search(root: &Node, target: i32) -> bool {
    let mut queue: VecDeque<&Node> = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        if node.value == target {
            return true;
        }
        for child in &node.children {
            queue.push_back(child);
        }
    }

    false
}
