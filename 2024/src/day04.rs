
use crate::Coord;

fn get_coord<'a>(space: &'a Vec<&str>, coord: Coord) -> Option<char> {
    space.get(coord.y as usize)?.chars().nth(coord.x as usize)
}

fn check(space: &Vec<&str>, start: Coord, dir: Coord) -> bool {
    if (start + dir*3).x < 0 || (start + dir*3).x >= space.get(0).unwrap().len() as isize ||
        (start + dir*3).y < 0 || (start + dir*3).y >= space.len() as isize
        {return false;}

    let check = "XMAS";
    for i in 0..4 {
        if get_coord(space, start + dir*i).unwrap() != check.chars().nth(i as usize).unwrap() {return false;}
    }
    true
}

fn part1(input: &Vec<&str>, right: isize, down: isize) {
    let mut result = 0;
    for x in 0..right {
        for y in 0..down {
            if get_coord(&input, Coord{x,y}) != Some('X') {continue};
            for xdiff in -1..=1 {
                for ydiff in -1..=1 {
                    if xdiff != 0 || ydiff != 0 {
                        if check(&input, Coord{x,y}, Coord{x:xdiff,y:ydiff}) {result += 1;}
                    }
                }
            }
        }
    }
    println!("{result}")
}

fn check2(input: &Vec<&str>, coord: Coord) -> bool {
    if get_coord(input, coord) != Some('A') {return false;}
    let offsets = vec![Coord{x:-1,y:-1}, Coord{x:-1,y:1}, Coord{x:1,y:1}, Coord{x:1,y:-1}];    
    let characters: Vec<char> = offsets.iter().filter_map(|&c| get_coord(input, c+coord)).collect();
    if characters.iter().filter(|&c| c == &'S').count() != 2 || characters.iter().filter(|&c| c == &'M').count() != 2 {return false;}
    characters.get(0) == characters.get(1) || characters.get(0) == characters.get(3)
}

fn part2(input: &Vec<&str>, right: isize, down: isize) {
    let mut result = 0;
    for x in 0..right {
        for y in 0..down {
            if check2(&input, Coord{x,y}) {result += 1;}
        }
    }
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