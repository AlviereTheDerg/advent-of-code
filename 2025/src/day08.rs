
use std::collections::HashMap;

fn dist(a: (i64,i64,i64), b:(i64,i64,i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn find(parents: &mut HashMap<(i64,i64,i64),(i64,i64,i64)>, node: (i64,i64,i64)) -> (i64,i64,i64) {
    let this_parent = *parents.get(&node).unwrap();
    if this_parent != node {
        let new_parent = find(parents, this_parent);
        parents.insert(node, new_parent);
        return new_parent;
    } else {
        node
    }
}

fn union(parents: &mut HashMap<(i64,i64,i64),(i64,i64,i64)>, a: (i64,i64,i64), b:(i64,i64,i64)) {
    let a = find(parents, a);
    let b = find(parents ,b);
    parents.insert(a, b);
}

fn part1(boxes: &Vec<(i64,i64,i64)>) {
    let mut box_pairs: Vec<(i64, (i64,i64,i64), (i64,i64,i64))> = Vec::new();
    let mut scanned_boxes: HashMap<(i64,i64,i64), (i64,i64,i64)> = HashMap::new();
    for &junction in boxes {
        for &other_junction in scanned_boxes.keys() {
            box_pairs.push((dist(junction, other_junction), junction, other_junction));
        }
        scanned_boxes.insert(junction, junction);
    }

    box_pairs.sort();
    let mut box_pairs_iter = box_pairs.iter().map(|(_, a, b)| (a,b));
    for _ in 0..1000 {
        let (&a, &b) = box_pairs_iter.next().unwrap();
        union(&mut scanned_boxes, a, b);
    }

    let mut circuit_sizes: HashMap<(i64,i64,i64), i64> = HashMap::new();
    for &junction in boxes {
        *circuit_sizes.entry(find(&mut scanned_boxes, junction)).or_insert(0) += 1;
    }

    let mut circuit_sizes_vec: Vec<_> = circuit_sizes.values().collect();
    circuit_sizes_vec.sort(); circuit_sizes_vec.reverse();
    let result = *circuit_sizes_vec.get(0).unwrap() * *circuit_sizes_vec.get(1).unwrap() * *circuit_sizes_vec.get(2).unwrap();
    println!("{}", result);
}

pub fn main() {
    let input = crate::grab_input("day08");
    let boxes: Vec<_> = input.split_whitespace()
        .map(|line| {
            let nums: Vec<i64> = line.splitn(3, ',').map(|val| val.parse().unwrap()).collect();
            (*nums.get(0).unwrap(), *nums.get(1).unwrap(), *nums.get(2).unwrap())
        })
        .collect();

    part1(&boxes);
}