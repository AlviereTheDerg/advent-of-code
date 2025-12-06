

fn part1(operations: &Vec<Vec<&str>>) {
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

pub fn main() {
    let input = crate::grab_input("day06");
    let data: Vec<Vec<&str>> = input.split("\n").filter(|str| !str.is_empty())
        .map(|line| {
            line.split_whitespace().collect()
        })
        .collect();
    let flipways: Vec<Vec<&str>> = (0..data.first().unwrap().len()).map(|index| {
            data.iter().map(|row| *row.get(index).unwrap()).collect()
        })
        .collect();

    part1(&flipways);
}