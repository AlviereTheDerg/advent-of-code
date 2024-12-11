use std::collections::HashMap;


fn blink_transform(stone: isize) -> Vec<isize> {
    if stone == 0 {
        vec![1]
    } else if (stone.ilog10()+1) % 2 == 0 {
        vec![stone / 10isize.pow((stone.ilog10()+1) / 2), stone % 10isize.pow((stone.ilog10()+1) / 2)]
    } else {
        vec![stone * 2024]
    }
}

fn part1(stones: &Vec<isize>) {
    let mut stones = stones.clone();
    for _ in 0..25 {
        stones = stones.into_iter()
            .map(|stone| blink_transform(stone).into_iter())
            .flat_map(std::convert::identity)
            .collect()
    }
    let result = stones.len();
    println!("{result}");
}

fn part2(stones: &Vec<isize>) {
    let mut stone_counts = HashMap::<isize, usize>::new();
    for stone in stones.iter() {
        stone_counts.entry(*stone)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    for _ in 0..75 {
        let mut next_stones = HashMap::<isize, usize>::new();

        for (stone, count) in stone_counts.iter() {
            for stone in blink_transform(*stone) {
                next_stones.entry(stone)
                    .and_modify(|v| *v += count)
                    .or_insert(*count);
            }
        }

        stone_counts = next_stones;
    }
    let result: usize = stone_counts.values().sum();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day11");
    let stones = input.split_whitespace()
        .filter_map(|stone| 
            if !stone.is_empty() {
                stone.parse::<isize>().ok()
            } else {None}
        )
        .collect::<Vec<isize>>();
    part1(&stones);
    part2(&stones);
}