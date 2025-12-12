

pub fn main() {
    let input = crate::grab_input("day12");

    let mut presents: Vec<&str> = Vec::new();
    let mut trees: Vec<&str> = Vec::new();
    for block in input.split("\n\n") {
        if let Some((_, block)) = block.split_once(":\n") {
            presents.push(block);
        } else {
            trees.extend(block.split("\n").filter(|line| !line.is_empty()));
        }
    }

    let present_sizes: Vec<usize> = presents.iter().map(|&present| present.chars().filter(|ch| *ch == '#').count()).collect();
    let mut result = 0;
    let mut smallest_positive: i64 = 100000000000000000;
    for tree in trees {
        let (space, counts) = tree.split_once(": ").unwrap();
        let space = {
            let (a,b) = space.split_once("x").unwrap();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        };

        let counts: Vec<usize> = counts.split_whitespace().map(|val| val.parse().unwrap()).collect();

        let cumulative: usize = present_sizes.iter().enumerate()
            .map(|(index, size)| counts.get(index).unwrap() * size)
            .sum();
        println!("{} {} {}", space, cumulative, (space as i64) - (cumulative as i64));
        if space > cumulative {
            smallest_positive = smallest_positive.min((space as i64) - (cumulative as i64));
            result += 1
        }
    }
    println!("{}", result); // can't believe that fucking worked
}