
use std::collections::HashMap;

fn dfs_path_counts<'a>(nodes: &HashMap<&'a str, Vec<&'a str>>, hits: &mut HashMap<&'a str, i64>, here: &'a str, goal: &str) -> i64 {
    if here == goal {return 1;}
    if let Some(hit_count) = hits.get(here) {return *hit_count;} 
    let mut result = 0;
    for &child in nodes.get(here).unwrap() {
        result += dfs_path_counts(nodes, hits, child, goal);
    }
    hits.insert(here, result);
    result
}

fn part1(nodes: &HashMap<&str, Vec<&str>>) {
    let result = dfs_path_counts(nodes, &mut HashMap::new(), "you", "out");
    println!("{}", result);
}

fn dfs_path_counts_with_node_hits<'a>(
    nodes: &HashMap<&'a str, Vec<&'a str>>, 
    hits: &mut HashMap<(&'a str, bool, bool), i64>, 
    here: &'a str, 
    goal: &str, 
    dac_hit: bool, 
    fft_hit: bool
) -> i64 {
    if here == goal {
        return if dac_hit && fft_hit {1} else {0};
    }
    if let Some(hit_count) = hits.get(&(here, dac_hit, fft_hit)) {return *hit_count;}
    let mut result = 0;
    for &child in nodes.get(here).unwrap() {
        result += dfs_path_counts_with_node_hits(
            nodes, 
            hits,
            child, 
            goal, 
            dac_hit || here == "dac", 
            fft_hit || here == "fft"
        );
    }
    hits.insert((here, dac_hit, fft_hit), result);
    result
}

fn part2(nodes: &HashMap<&str, Vec<&str>>) {
    let result = dfs_path_counts_with_node_hits(nodes, &mut HashMap::new(), "svr", "out", false, false);
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
    part2(&nodes);
}