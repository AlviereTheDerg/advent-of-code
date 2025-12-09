
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dist(boxes: &Vec<(i64,i64,i64)>, a: usize, b: usize) -> i64 {
    let a = boxes.get(a).unwrap();
    let b = boxes.get(b).unwrap();
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn find(parents: &mut Vec<usize>, node: usize) -> usize {
    let this_parent = *parents.get(node).unwrap();
    if this_parent != node {
        let new_parent = find(parents, this_parent);
        parents[node] = new_parent;
        return new_parent;
    } else {
        node
    }
}

fn union(parents: &mut Vec<usize>, sizes: &mut Vec<i64>, a: usize, b:usize) -> i64 {
    let a = find(parents, a);
    let b = find(parents ,b);
    if a != b {
        parents[a] = b; // a -> b (b is new root)
        let child_size = *sizes.get(a).unwrap();
        *sizes.get_mut(b).unwrap() += child_size; // add a's size to b
    }
    // if b was not originally root, b is now root
    *sizes.get(b).unwrap()
}

pub fn main() {
    let input = crate::grab_input("day08");
    let boxes: Vec<_> = input.split_whitespace()
        .map(|line| {
            let nums: Vec<i64> = line.splitn(3, ',').map(|val| val.parse().unwrap()).collect();
            (*nums.get(0).unwrap(), *nums.get(1).unwrap(), *nums.get(2).unwrap())
        })
        .collect();

    let mut heap = BinaryHeap::new(); // min-heap storing (edge^2, id1, id2)
    for junction in 0..boxes.len() {
        for other_junction in 0..junction {
            heap.push(Reverse(
                (dist(&boxes, junction, other_junction), junction, other_junction)
            ));
        }
    }
    let mut parents: Vec<usize> = (0..boxes.len()).map(|v| v).collect(); // this.id -> parent.id
    let mut sizes: Vec<i64> = (0..boxes.len()).map(|_| 1).collect(); // this.id -> this.size
    
    for _ in 0..1000 {
        let (_, a, b) = heap.pop().unwrap().0;
        union(&mut parents, &mut sizes, a, b);
    }
    
    let mut circuit_sizes_vec: Vec<_> = sizes.clone();
    circuit_sizes_vec.sort();
    let result: i64 = circuit_sizes_vec[circuit_sizes_vec.len() - 3..].iter().product();
    println!("{}", result); // part 1
    
    let goal = boxes.len() as i64;
    loop {
        let (_, a, b) = heap.pop().unwrap().0;
        if union(&mut parents, &mut sizes, a, b) == goal {
            let a = boxes.get(a).unwrap();
            let b = boxes.get(b).unwrap();
            println!("{}", a.0 * b.0);
            break;
        }
    }
}