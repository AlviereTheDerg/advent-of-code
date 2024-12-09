
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

    println!("{input:?}");
    let mut total_block_length = 0;
    for (index, item) in input.iter().enumerate() {
        if index % 2 == 0 {total_block_length += item}
    }
    println!("{total_block_length}");
    
    //let total_block_length = input.iter().enumerate()
    //    .filter_map(|(index, &value)| if index & 2 == 0 {Some(value)} else {None})
    //    .sum::<u32>();
    //println!("{total_block_length}");

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
    //println!("{}", file_blocks.iter().map(|v| v.to_string()).collect::<String>());

    let checksum = file_blocks.iter().enumerate().map(|(index, value)| index * value).sum::<usize>();
    println!("{checksum}");
}

pub fn main() {
    let input = crate::grab_input("day09")
        .split_whitespace().next().unwrap().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    part1(&input);
}