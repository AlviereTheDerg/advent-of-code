
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
}