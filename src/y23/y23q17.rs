use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::utils;

pub fn y23q17q1(input:Vec<String>) -> String {
    let a = utils::inp_to_array2(input)
        .map(|c| c.to_digit(10).unwrap() as usize);
    let goal_x = a.nrows() - 1;
    let goal_y = a.ncols() - 1;
    // Node(x, y, true/false = horizontal/vertical)
    // we implement the restriction by adding neighbors in triplets in each 90° direction
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
    struct Node(usize, usize, bool);
    let mut boundary_nodes:BinaryHeap<Reverse<(usize, Node)>> = BinaryHeap::from([
        Reverse((a[[0,1]], Node(0, 1, true))),
        Reverse((a[[0,1]]+a[[0,2]], Node(0, 2, true))),
        Reverse((a[[0,1]]+a[[0,2]]+a[[0,3]], Node(0, 3, true))),
        Reverse((a[[1,0]], Node(1, 0, false))),
        Reverse((a[[1,0]]+a[[2,0]], Node(2, 0, false))),
        Reverse((a[[1,0]]+a[[2,0]]+a[[3,0]], Node(3, 0, false)))
    ]);

    let mut visited_nodes:HashMap<Node, usize> = HashMap::from([
        (Node(0, 0, true), 0),
        (Node(0, 0, false), 0)
    ]);

    while let Some(Reverse(node)) = boundary_nodes.pop() {
        // println!(">>> ex >>> {:?}", node);
        // println!("current visited nodes {:?}", visited_nodes);
        let x = node.1.0;
        let y = node.1.1;
        let dir = node.1.2;
        let val = node.0;
        // goal reached
        if x == goal_x && y == goal_y {
            return val.to_string();
        }
        if visited_nodes.contains_key(&Node(x, y, dir)) {
            continue;
        }
        visited_nodes.insert(Node(x, y, dir), val);
        // otherwise add all 90° nodes unless path with same direction already visited
        if dir {
            let mut val_new = val;
            let x_new_vec:Vec<usize> = (1..=3).map(|i| x.saturating_sub(i))
                .filter(|&x_new| x_new < x).collect();
            for x_new in x_new_vec {
                val_new += a[[x_new, y]];
                if !visited_nodes.contains_key(&Node(x_new, y, !dir)) {
                    boundary_nodes.push(Reverse((val_new, Node(x_new, y, !dir))));
                }
            }
            val_new = val;
            for x_new in vec![x+1, x+2, x+3] {
                if x_new <= goal_x {
                    val_new += a[[x_new, y]];
                    if !visited_nodes.contains_key(&Node(x_new, y, !dir)) {
                        boundary_nodes.push(Reverse((val_new, Node(x_new, y, !dir))));
                    }
                }
            }
        } else {
            let mut val_new = val;
            let y_new_vec:Vec<usize> = (1..=3).map(|i| y.saturating_sub(i))
                .filter(|&y_new| y_new < y).collect();
            for y_new in y_new_vec {
                val_new += a[[x, y_new]];
                if !visited_nodes.contains_key(&Node(x, y_new, !dir)) {
                    boundary_nodes.push(Reverse((val_new, Node(x, y_new, !dir))));
                }
            }
            val_new = val;
            for y_new in vec![y+1, y+2, y+3] {
                if y_new <= goal_y {
                    val_new += a[[x, y_new]];
                    if !visited_nodes.contains_key(&Node(x, y_new, !dir)) {
                        boundary_nodes.push(Reverse((val_new, Node(x, y_new, !dir))));
                    }
                }
            }
        }
    }
    "-1".to_string()
}

pub fn y23q17q2(input:Vec<String>) -> String {
    let a = utils::inp_to_array2(input)
        .map(|c| c.to_digit(10).unwrap() as usize);
    let goal_x = a.nrows() - 1;
    let goal_y = a.ncols() - 1;
    // Node(x, y, true/false = horizontal/vertical)
    // we implement the restriction by adding neighbors in triplets in each 90° direction
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
    struct Node(usize, usize, bool);
    let mut boundary_nodes:BinaryHeap<Reverse<(usize, Node)>> = BinaryHeap::new();
    // populate with initial nodes
    let mut x_add = a[[1, 0]] + a[[2, 0]] + a[[3, 0]];
    for x in 4..=10 {
        x_add += a[[x, 0]];
        boundary_nodes.push(Reverse((x_add, Node(x, 0, false))));
    }
    let mut y_add = a[[0, 1]] + a[[0, 2]] + a[[0, 3]];
    for y in 4..=10 {
        y_add += a[[0, y]];
        boundary_nodes.push(Reverse((y_add, Node(0, y, true))));
    }

    let mut visited_nodes:HashMap<Node, usize> = HashMap::from([
        (Node(0, 0, true), 0),
        (Node(0, 0, false), 0)
    ]);

    while let Some(Reverse(node)) = boundary_nodes.pop() {
        // println!(">>> ex >>> {:?}", node);
        // println!("current visited nodes {:?}", visited_nodes);
        let x = node.1.0;
        let y = node.1.1;
        let dir = node.1.2;
        let val = node.0;
        // goal reached
        if x == goal_x && y == goal_y {
            return val.to_string();
        }
        if visited_nodes.contains_key(&Node(x, y, dir)) {
            continue;
        }
        visited_nodes.insert(Node(x, y, dir), val);
        // otherwise add all 90° nodes unless path with same direction already visited
        if dir {
            let x_new_vec:Vec<usize> = (4..=10).filter_map(|i| x.checked_sub(i))
                .take_while(|&x_new| x_new < x).collect();
            if x_new_vec.len() > 0 {
                let mut val_new = val + a[[x-1, y]] + a[[x-2, y]] + a[[x-3, y]];
                for x_new in x_new_vec {
                    val_new += a[[x_new, y]];
                    if !visited_nodes.contains_key(&Node(x_new, y, !dir)) {
                        boundary_nodes.push(Reverse((val_new, Node(x_new, y, !dir))));
                    }
                }
            }
            if x+4 <= goal_x {
                let mut val_new = val + a[[x+1, y]] + a[[x+2, y]] + a[[x+3, y]];
                for x_new in vec![x+4, x+5, x+6, x+7, x+8, x+9, x+10] {
                    if x_new <= goal_x {
                        val_new += a[[x_new, y]];
                        if !visited_nodes.contains_key(&Node(x_new, y, !dir)) {
                            boundary_nodes.push(Reverse((val_new, Node(x_new, y, !dir))));
                        }
                    }
                }
            }
        } else {
            let y_new_vec:Vec<usize> = (4..=10).filter_map(|i| y.checked_sub(i))
                .take_while(|&y_new| y_new < y).collect();
            if y_new_vec.len() > 0 {
                let mut val_new = val + a[[x, y-1]] + a[[x, y-2]] + a[[x, y-3]];
                for y_new in y_new_vec {
                    val_new += a[[x, y_new]];
                    if !visited_nodes.contains_key(&Node(x, y_new, !dir)) {
                        boundary_nodes.push(Reverse((val_new, Node(x, y_new, !dir))));
                    }
                }
            }
            if y+4 <= goal_y {
                let mut val_new = val + a[[x, y+1]] + a[[x, y+2]] + a[[x, y+3]];
                for y_new in vec![y+4, y+5, y+6, y+7, y+8, y+9, y+10] {
                    if y_new <= goal_y {
                        val_new += a[[x, y_new]];
                        if !visited_nodes.contains_key(&Node(x, y_new, !dir)) {
                            boundary_nodes.push(Reverse((val_new, Node(x, y_new, !dir))));
                        }
                    }
                }
            }
        }
    }
    "-1".to_string()
}