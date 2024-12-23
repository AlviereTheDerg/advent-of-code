
use std::collections::{HashMap, HashSet};

fn part1(input: &HashMap<&str, HashSet<&str>>) {
    let mut found_trios: HashSet<(&str, &str, &str)> = HashSet::new();
    for (&first, first_adjacents) in input.iter() {
        for &second in first_adjacents.iter() {
            let second_adjacents = input.get(second).unwrap();

            let third_options = first_adjacents.intersection(second_adjacents);
            for &third in third_options {
                let mut option_trio = vec![first, second, third];
                option_trio.sort();
                // hacky way of making sure the 3 values are in same order
                found_trios.insert((option_trio[0], option_trio[1], option_trio[2]));
            }
        }
    }
    let mut result = 0;
    for (first, second, third) in found_trios {
        if 
            Some('t') == first.chars().nth(0) ||
            Some('t') == second.chars().nth(0) ||
            Some('t') == third.chars().nth(0)
        {
            result += 1;
        }
    }
    println!("{result}");
}

fn part2(input: &HashMap<&str, HashSet<&str>>) {
    let mut clusters: Vec<(Vec<&str>, &str)> = input.keys()
        .map(|&nucleation| {
            (Vec::new(), nucleation)
        })
        .collect();
    
    let mut largest_cluster: Vec<&str> = Vec::new();
    let mut visited: HashSet<Vec<&str>> = HashSet::new();
    while let Some((mut cluster, addition)) = clusters.pop() {
        cluster.push(addition);

        // don't re-search same cluster over and over
        cluster.sort();
        if visited.contains(&cluster) {continue;}
        visited.insert(cluster.clone());

        // update largest found cluster as need be
        if cluster.len() > largest_cluster.len() {
            largest_cluster = cluster.clone();
        }

        let potential_expansions = input.get(addition).unwrap().iter()
            .filter(|&&node|
                cluster.iter()
                    .map(|key| input.get(key).unwrap())
                    .all(|set| set.contains(node))
            );
        for &neighbour in potential_expansions {
            clusters.push((cluster.clone(), neighbour));
        }
    }

    largest_cluster.sort();
    let result = largest_cluster.join(",");
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day23");
    let edges: Vec<Vec<&str>> = input.split_whitespace()
        .filter_map(|line|
            if !line.is_empty() {
                Some(line.split("-").collect())
            } else {None}
        )
        .collect();

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for edge in edges.iter() {
        let source = *edge.get(0).unwrap();
        let dest = *edge.get(1).unwrap();

        // add source->dest
        graph.entry(source)
            .and_modify(|adjacency| {adjacency.insert(dest);} )
            .or_insert({
                let mut adjacency = HashSet::new();
                adjacency.insert(dest);
                adjacency
            });
        
        // add dest->source
        graph.entry(dest)
            .and_modify(|adjacency| {adjacency.insert(source);} )
            .or_insert({
                let mut adjacency = HashSet::new();
                adjacency.insert(source);
                adjacency
            });
    }

    part1(&graph);
    part2(&graph)
}