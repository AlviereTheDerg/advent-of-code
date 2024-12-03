
fn part1(input: &str) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for (index, entry) in input.split_whitespace().enumerate() {
        if index % 2 == 0 {&mut list1} else {&mut list2}
            .push(entry.parse::<isize>().unwrap());
    }

    list1.sort(); list2.sort();
    let result: isize = list1.iter().zip(list2.iter()).map(|(a,b)| (a-b).abs()).sum();
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day01");
    part1(&input);
}