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

fn stones_after_n_blinks(stones: &Vec<isize>, blinks: usize) -> usize {
    let mut stone_counts = HashMap::<isize, usize>::new();
    for stone in stones.iter() {
        stone_counts.entry(*stone)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    for _ in 0..blinks {
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
    stone_counts.values().sum()
}

fn part1(stones: &Vec<isize>) {
    let result = stones_after_n_blinks(stones, 25);
    println!("{result}");
}

fn part2(stones: &Vec<isize>) {
    let result = stones_after_n_blinks(stones, 75);
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