
use std::collections::{HashMap, HashSet};
use crate::{Coord, New};

fn part1(charsmap: &mut HashMap<Coord, char>, rows: usize, start: Coord, columns: usize) {
    let mut splitters_hit: HashSet<Coord> = HashSet::new();
    let mut beam_heads: HashSet<Coord> = HashSet::from([start]);

    let beam_propagation = Coord{x:0, y:1};
    let splitter_propagation = vec![Coord{x:-1, y:0}, Coord{x:1, y:0}];

    loop {
        let mut new_beam_heads: HashSet<Coord> = HashSet::new();

        for beam in beam_heads {
            if beam.y > rows as isize {continue;}
            if let Some(hit) = charsmap.get(&beam) && hit == &'^' {
                splitters_hit.insert(beam);
                new_beam_heads.extend(splitter_propagation.iter().map(|&coord| coord + beam));
            } else {
                new_beam_heads.insert(beam + beam_propagation);
                charsmap.insert(beam, '|');
            }
        }
        
        if new_beam_heads.len() == 0 {break;}
        beam_heads = new_beam_heads;
    }

    println!("{}", splitters_hit.len());
    for y in 0..rows {
        for x in 0..columns {
            print!("{}", charsmap.get(&Coord::new(x,y)).unwrap_or(&'.'));
        }
        println!("");
    }
}

pub fn main() {
    let input = crate::grab_input("day07");
    let mut charsmap: HashMap<Coord, char> = input.split_whitespace().enumerate()
        .flat_map(|(row_index, row)| {
            row.chars().enumerate().map(move |(column_index, ch)| {
                (Coord::new(column_index, row_index), ch)
            })
        })
        .filter(|(_, ch)| *ch != '.')
        .collect();

    let rows = input.split_whitespace().count();
    let columns = input.split_whitespace().next().unwrap().len();
    let start = *charsmap.iter().filter_map(|(coord, ch)| if *ch == 'S' {Some(coord)} else {None}).next().unwrap();

    part1(&mut charsmap, rows, start, columns);
}