
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

fn is_point_inside(lines: &Vec<(Coord, Coord)>, x: isize, y: isize) -> bool {
    lines.iter()
        .filter(|(a,b)| a.x == b.x) // just vertical lines
        .filter(|(a, _)| a.x < x) // just ones left of x
        .filter(|(a, b)| {
            let y1 = a.y.min(b.y);
            let y2 = a.y.max(b.y);
            y1 < y && y < y2
        }) // just ones where y overlaps
        .count() % 2 == 1
}

fn does_line_intersect(line: &(Coord, Coord), x1: isize, x2: isize, y1: isize, y2: isize) -> bool {
    // check vertical line pairs
    if line.0.x == line.1.x && x1 == x2 {
        // if they're in the same column and the Y values overlap
        let y_low = line.0.y.min(line.1.y);
        let y_high = line.0.y.max(line.1.y);
        return line.0.x == x1 && ((y_low < y1 && y1 < y_high) || (y_low < y2 && y2 < y_high)); 
    }
    // check horizontal line pairs
    if line.0.y == line.1.y && y1 == y2 {
        // if they're in the same row and the X values overlap
        let x_low = line.0.x.min(line.1.x);
        let x_high = line.0.x.max(line.1.x);
        return line.0.y == y1 && ((x_low < x1 && x1 < x_high) || (x_low < x2 && x2 < x_high)); 
    }
    let (hx1, hx2, hy, vx, vy1, vy2) = if x1 == x2 { // line=horizontal
        (line.0.x.min(line.1.x), line.0.x.max(line.1.x), line.0.y, x1, y1.min(y2), y1.max(y2))
    } else { // coords=horizontal
        (x1.min(x2), x1.max(x2), y1, line.0.x, line.0.y.min(line.1.y), line.0.y.max(line.1.y))
    };

    // if the vertical line's column falls in the horizontal space of the horizontal line
    (hx1 < vx && vx < hx2)
    // and the horizontal line's row falls in the space of the vertical line
    && (vy1 < hy && hy < vy2)
}

fn is_valid_polygon(lines: &Vec<(Coord, Coord)>, x1: isize, x2: isize, y1: isize, y2: isize) -> bool {
    let test_points = vec![(x1,y1), (x1,y2), (x2,y1), (x2,y2)];
    for (x,y) in test_points.iter() {
        if !is_point_inside(lines, *x, *y) {return false;}
    }
    let test_lines: Vec<_> = (0..test_points.len())
        .map(|v| (v, (v+1) % test_points.len()))
        .map(|(a,b)| (test_points.get(a).unwrap(), test_points.get(b).unwrap()))
        .collect();
    for ((x1, y1), (x2, y2)) in test_lines {
        for line in lines {
            if does_line_intersect(line, *x1, *x2, *y1, *y2) {return false;}
        }
    }
    true
}

fn part2(red_tiles: &Vec<Coord>) {
    // get all line segments
    let lines: Vec<(Coord, Coord)> = (0..red_tiles.len())
        .map(|x| (x,(x+1)%red_tiles.len()))
        .map(|(a,b)| (*red_tiles.get(a).unwrap(), *red_tiles.get(b).unwrap()))
        .collect();

    let doubled_lines: Vec<(Coord, Coord)> = lines.iter()
        .map(|(a,b)| {
            (*a*2, *b*2)
        })
        .collect();

    let mut result = 0;
    for (index, a) in red_tiles.iter().enumerate() {
        for b in &red_tiles[..index] {
            // doubling the value of all coordinates won't change the collision detection
            // but can then use that doubling to have the test polygon on .5s without futzing with floating points
            let x1 = a.x.min(b.x) * 2 + 1;
            let x2 = a.x.max(b.x) * 2 - 1;
            let y1 = a.y.min(b.y) * 2 + 1;
            let y2 = a.y.max(b.y) * 2 - 1;

            if is_valid_polygon(&doubled_lines, x1, x2, y1, y2) {
                result = result.max(((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1));
            }
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
    part2(&red_tiles);
}