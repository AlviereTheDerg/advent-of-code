
use crate::{Coord, New};
use regex::Regex;

fn position_after_n_seconds(guard: &(Coord, Coord), bounds: &Coord, seconds: isize) -> Coord {
    let position = guard.0 + guard.1 * seconds;
    Coord::new(
        ((position.x % bounds.x) + bounds.x) % bounds.x,
        ((position.y % bounds.y) + bounds.y) % bounds.y
    )
}

fn part1(input: &Vec<(Coord, Coord)>, bounds: &Coord) {
    let robots = input.iter()
        .map(|guard| position_after_n_seconds(guard, bounds, 100))
        .collect::<Vec<Coord>>();

    println!("{robots:?}");

    let half_bounds = Coord::new(bounds.x / 2, bounds.y / 2);
    let x_shift = Coord::new(half_bounds.x + 1, 0isize);
    let y_shift = Coord::new(0isize, half_bounds.y + 1);
    let upper_left = robots.iter().filter(|c| c.within_bounds(&half_bounds)).count();
    let upper_right = robots.iter().map(|c| *c - x_shift).filter(|c| c.within_bounds(&half_bounds)).count();
    let lower_left = robots.iter().map(|c| *c - y_shift).filter(|c| c.within_bounds(&half_bounds)).count();
    let lower_right = robots.iter().map(|c| (*c - x_shift) - y_shift).filter(|c| c.within_bounds(&half_bounds)).count();
    let result = upper_left * upper_right * lower_left * lower_right;
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day14");
    let bounds = Coord::new(101isize,103isize);
    let mut robots = Vec::<(Coord, Coord)>::new();
    let extractor = Regex::new(r#"p=(\d+),(\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    for capture in extractor.captures_iter(&input) {
        let (_, [px, py, vx, vy]) = capture.extract();
        let px = px.parse::<isize>().unwrap();
        let py = py.parse::<isize>().unwrap();
        let vx = vx.parse::<isize>().unwrap();
        let vy = vy.parse::<isize>().unwrap();
        robots.push((
            Coord::new(px, py),
            Coord::new(vx, vy)
        ));
    }

    part1(&robots, &bounds);
}