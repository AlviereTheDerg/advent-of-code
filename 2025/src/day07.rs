
use std::collections::{HashMap, HashSet};
use crate::{Coord, New};

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

    let mut splitters_hit: HashSet<Coord> = HashSet::new();
    let mut beam_heads: HashMap<Coord, usize> = HashMap::from([(start, 1usize)]);
    let mut retired_beam_heads: HashMap<Coord, usize> = HashMap::new();

    let beam_propagation = Coord{x:0, y:1};
    let splitter_propagation = vec![Coord{x:-1, y:0}, Coord{x:1, y:0}];

    loop {
        let mut new_beam_heads: HashMap<Coord, usize> = HashMap::new();

        for (beam, count) in beam_heads {
            if beam.y > rows as isize {
                *retired_beam_heads.entry(beam).or_insert(0) += count;
                continue;
            }
            if let Some(hit) = charsmap.get(&beam) && hit == &'^' {
                splitters_hit.insert(beam);
                for split in splitter_propagation.iter().map(|&coord| coord + beam) {
                    *new_beam_heads.entry(split).or_insert(0) += count;
                }
            } else {
                *new_beam_heads.entry(beam + beam_propagation).or_insert(0) += count;
                charsmap.insert(beam, '|');
            }
        }
        
        if new_beam_heads.len() == 0 {break;}
        beam_heads = new_beam_heads;
    }
    
    // beam printing!
    for y in 0..rows {
        for x in 0..columns {
            print!("{}", charsmap.get(&Coord::new(x,y)).unwrap_or(&'.'));
        }
        println!("");
    }

    // actual part 1/2 display
    println!("{}", splitters_hit.len());
    println!("{}", retired_beam_heads.values().sum::<usize>());
}