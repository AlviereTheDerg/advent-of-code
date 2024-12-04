
use crate::Coord;

fn get_coord<'a>(space: &'a Vec<&str>, coord: Coord) -> char {
    space.get(coord.y as usize).unwrap().chars().nth(coord.x as usize).unwrap()
}

fn check(space: &Vec<&str>, start: Coord, dir: Coord) -> bool {
    if (start + dir*3).x < 0 || (start + dir*3).x >= space.get(0).unwrap().len() as isize ||
        (start + dir*3).y < 0 || (start + dir*3).y >= space.len() as isize
        {return false;}

    let check = "XMAS";
    for i in 0..4 {
        if get_coord(space, start + dir*i) != check.chars().nth(i as usize).unwrap() {return false;}
    }
    true
}

fn part1(input: &str) {
    let input: Vec<&str> = input.split_whitespace().collect();
    let right = input.get(0).unwrap().len() as isize;
    let down = input.len() as isize;

    let mut result = 0;
    for x in 0..right {
        for y in 0..down {
            if get_coord(&input, Coord{x,y}) != 'X' {continue};
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

pub fn main() {
    let input = crate::grab_input("day04");
    part1(&input);
}