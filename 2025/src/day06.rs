

fn part1(input: &str) {
    let data: Vec<Vec<&str>> = input.split("\n").filter(|str| !str.is_empty())
        .map(|line| {
            line.split_whitespace().collect()
        })
        .collect();
    let operations: Vec<Vec<&str>> = (0..data.first().unwrap().len()).map(|index| {
            data.iter().map(|row| *row.get(index).unwrap()).collect()
        })
        .collect();
    let mut result: u64 = 0;

    for operation in operations {
        let mut contents = operation.iter().rev();
        let op = *contents.next().unwrap();
        result += contents.map(|val| val.parse::<u64>().unwrap())
            .fold(if op == "*" {1u64} else {0u64}, |acc, x| {
                if op == "*" {
                    acc * x
                } else {
                    acc + x
                }
            });
    }

    println!("{}", result);
}

fn part2(input: &str) {
    let mut rows: Vec<_> = input.split("\n").filter(|str| !str.is_empty()).map(|row| row.chars()).collect();
    let columns: Vec<_> = (0..input.split_once("\n").unwrap().0.len())
        .map(|_| {
            rows.iter_mut().map(|row| row.next().unwrap()).collect::<String>()
        })
        .collect();
    let blocks: Vec<_> = columns.split(|column| column.trim().is_empty()).collect();

    let mut result: u64 = 0;

    for block in blocks {
        let op = block[0].chars().last().unwrap();
        let mut acc: u64 = if op == '*' {1} else {0};

        for line in block {
            let stripped_line = line.chars().filter(|ch| "0123456789".contains(*ch)).collect::<String>();
            let val: u64 = stripped_line.parse().unwrap();
            if op == '*' {
                acc *= val;
            } else {
                acc += val;
            }
        }

        result += acc;
    }

    println!("{}", result);
}

pub fn main() {
    let input = crate::grab_input("day06");

    part1(&input);
    part2(&input);
}