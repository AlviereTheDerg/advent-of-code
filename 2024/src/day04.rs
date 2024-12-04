
use crate::Coord;
use itertools::Itertools;

fn get_coord<'a>(space: &'a Vec<&str>, coord: Coord) -> Option<char> {
    space.get(coord.y as usize)?.chars().nth(coord.x as usize)
}

fn check(space: &Vec<&str>, start: Coord, dir: Coord) -> bool {
    let check = "XMAS";
    for i in 0..4 {
        if get_coord(space, start + dir*i) != check.chars().nth(i as usize) {return false;}
    }
    true
}

fn part1(input: &Vec<&str>, right: isize, down: isize) {
    let result = (0..right).cartesian_product(0..down)
        .map(|(x,y)| Coord{x,y})
        .cartesian_product(
            (-1..=1).cartesian_product(-1..=1).filter_map(|(x,y)| 
                if x != 0 || y != 0 {Some(Coord{x,y})} 
                else {None}
            )
        ).filter(|(start, dir)| check(&input, *start, *dir))
        .count();
    println!("{result}")
}

fn check2(input: &Vec<&str>, coord: Coord) -> bool {
    if get_coord(input, coord) != Some('A') {return false;}

    let characters: Vec<char> = vec![Coord{x:-1,y:-1}, Coord{x:-1,y:1}, Coord{x:1,y:1}, Coord{x:1,y:-1}].iter()
        .filter_map(|&c| get_coord(input, c+coord)).collect();

    characters.len() == 4 &&
    characters.iter().filter(|&c| c == &'S').count() == 2 &&
    characters.iter().filter(|&c| c == &'M').count() == 2 && 
    (
        characters.get(0) == characters.get(1) || 
        characters.get(0) == characters.get(3)
    )
}

fn part2(input: &Vec<&str>, right: isize, down: isize) {
    let result = (0..right).cartesian_product(0..down)
        .map(|(x,y)| Coord{x,y})
        .filter(|&coord| check2(&input, coord))
        .count();
    println!("{result}")
}

pub fn main() {
    let input = crate::grab_input("day04");
    let input: Vec<&str> = input.split_whitespace().collect();
    let right = input.get(0).unwrap().len() as isize;
    let down = input.len() as isize;
    part1(&input, right, down);
    part2(&input, right, down);
}