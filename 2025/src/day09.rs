
use crate::{Coord, New};

fn part1(red_tiles: &Vec<Coord>) {
    let mut result = 0;
    for (index, tile_a) in red_tiles.iter().enumerate() {
        for tile_b in &red_tiles[..index] {
            let possible_max = ((tile_a.x - tile_b.x).abs() + 1) * ((tile_a.y - tile_b.y).abs() + 1);
            if possible_max > result {result = possible_max}
        }
    }

    println!("{}", result);
}

pub fn main () {
    let input = crate::grab_input("day09");
    let red_tiles: Vec<_> = input.split_whitespace()
        .map(|line| {
            let (a,b) = line.split_once(',').unwrap();
            Coord::new(a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        })
        .collect();

    part1(&red_tiles);
}