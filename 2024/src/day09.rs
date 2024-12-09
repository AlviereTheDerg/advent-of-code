
use std::collections::HashSet;

fn part1(blocks: &Vec<(usize, usize, Option<usize>)>) {
    let expanded_blocks = blocks.iter()
        .map(|(length, index, id)| {
            (0usize..*length).map(move |i| {
                (index+i, *id)
            })
        })
        .flat_map(std::convert::identity)
        .collect::<Vec<_>>();
    
    let total_block_length = expanded_blocks.iter()
        .filter(|(_, id)| id != &None)
        .count();

    let mut non_empty_ids_reverse = expanded_blocks.iter().rev()
        .filter_map(|(_, id)| *id);

    let mut checksum = 0;
    for index in 0..total_block_length {
        let (index, id) = expanded_blocks.get(index).unwrap();
        checksum += index * if let Some(id) = id {
            *id
        } else {
            non_empty_ids_reverse.next().unwrap()
        }
    }
    println!("{checksum}");
}

fn part2(blocks: &Vec<(usize, usize, Option<usize>)>) {
    // block length, block index
    let mut empty_blocks = blocks.iter()
        .filter_map(|(length, index, id)| {
            if id == &None {
                Some((*length, *index))
            } else {None}
        })
        .collect::<HashSet<_>>();
    
    let mut checksum = 0; 
    for (length, index, id) in blocks.iter()
        .rev()
        .filter_map(|(length, index, id)| {
            if let Some(id) = id {
                Some((*length, *index, *id))
            } else {None}
        })
    {
        let selection = empty_blocks.iter()
            .filter(|(empty_length, empty_index)| { // get valid gaps
                *empty_length >= length &&
                *empty_index <= index
            })
            .reduce(|first, second| { // get first valid gap
                if first.1 < second.1 {first} else {second}
            });

        let placement_index = if let Some(&selection) = selection { // space to fill
            empty_blocks.remove(&selection);
            
            // if the entire space isn't filled, re-add a smaller empty space after the filled
            if selection.0 > length { 
                empty_blocks.insert((selection.0 - length, selection.1 + length));
            }

            selection.1
        } else { // otherwise keep it in its current position
            index
        };
        checksum += id * length * (2 * placement_index + length - 1) / 2
    }
    println!("{checksum}");
}

pub fn main() {
    let input = crate::grab_input("day09")
        .split_whitespace().next().unwrap().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    // block length, starting index, ID (None if empty)
    let mut blocks = Vec::new();
    let mut total_index = 0usize;
    for (index, &length) in input.iter().enumerate() {
        blocks.push((
            length as usize,
            total_index,
            if index % 2 == 0 {Some(index / 2)} else {None}
        ));
        total_index += length as usize;
    }

    part1(&blocks);
    part2(&blocks);
}