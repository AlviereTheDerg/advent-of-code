use std::collections::HashSet;


fn part1(input: &Vec<u32>) {
    let mut forward_token_iter = input.iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            if index % 2 == 0 {
                Some((0..value).map(move |_| index / 2))
            } else {None}
        })
        .flat_map(std::convert::identity);

    let mut reverse_token_iter = input.iter()
        .enumerate()
        .rev()
        .filter_map(|(index, &value)| {
            if index % 2 == 0 {
                Some((0..value).map(move |_| index / 2))
            } else {None}
        })
        .flat_map(std::convert::identity);

    let total_block_length = input.iter().enumerate()
        .filter_map(|(index, &item)| if index % 2 == 0 {Some(item)} else {None})
        .sum::<u32>();

    let mut switch = false;
    let mut remaining_until_switch: u32 = 0;
    let mut value_iter = input.iter();
    let mut file_blocks = Vec::new();
    let mut index = 0;
    while index < total_block_length {
        if remaining_until_switch == 0 {
            remaining_until_switch = *value_iter.next().unwrap();
            switch = !switch;
            continue;
        }

        let block_id = if switch {forward_token_iter.next().unwrap()} else {reverse_token_iter.next().unwrap()};
        remaining_until_switch -= 1;
        index += 1;
        file_blocks.push(block_id);
    }

    let checksum = file_blocks.iter().enumerate().map(|(index, value)| index * value).sum::<usize>();
    println!("{checksum}");
}

fn part2(input: &Vec<u32>) {
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

    // block length, block index
    let mut empty_blocks = blocks.iter()
        .filter_map(|(length, index, id)| {
            if id == &None {
                Some((*length, *index))
            } else {None}
        })
        .collect::<HashSet<_>>();
    
    // block length, block index, block ID
    let full_blocks = blocks.iter()
        .filter_map(|(length, index, id)| {
            if let Some(id) = id {
                Some((*length, *index, *id))
            } else {None}
        })
        .collect::<Vec<_>>();
    
    let mut checksum = 0; 
    for (length, index, id) in full_blocks.iter().rev() {
        let selection = empty_blocks.iter()
            .filter(|(empty_length, empty_index)| { // get valid gaps
                empty_length >= length &&
                empty_index <= index
            })
            .reduce(|first, second| { // get first valid gap
                if first.1 < second.1 {first} else {second}
            });

        let placement_index = if let Some(&selection) = selection { // space to fill
            empty_blocks.remove(&selection);
            
            // if the entire space isn't filled, re-add a smaller empty space after the filled
            if selection.0 > *length { 
                empty_blocks.insert((selection.0 - length, selection.1 + length));
            }

            selection.1
        } else { // otherwise keep it in its current position
            *index
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
    part1(&input);
    part2(&input);
}