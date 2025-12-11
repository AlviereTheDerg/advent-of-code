
use std::collections::HashMap;

fn dfs_path_counts(nodes: &HashMap<&str, Vec<&str>>, here: &str, goal: &str) -> i64 {
    if here == goal {return 1;}
    let mut result = 0;
    for &child in nodes.get(here).unwrap() {
        result += dfs_path_counts(nodes, child, goal);
    }
    result
}

fn part1(nodes: &HashMap<&str, Vec<&str>>) {
    let result = dfs_path_counts(nodes, "you", "out");
    println!("{}", result);
}

pub fn main() {
    let input = crate::grab_input("day11");
    let nodes: HashMap<&str, Vec<&str>> = input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (node, children) = line.split_once(": ").unwrap();
            let children: Vec<&str> = children.split_whitespace().collect();
            (node, children)
        })
        .collect();

    part1(&nodes);
}