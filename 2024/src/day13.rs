
use regex::Regex;

fn part1(data_blocks: &Vec<Vec<isize>>) {
    let mut result = 0;
    for block in data_blocks {
        // ax*A + bx*B = px
        // ay*A + by*B = py
        // ay*ax*A + ay*bx*B = ay*px
        // ax*ay*A + ax*by*B = ax*py
        // ay*bx*B - ax*by*B = ay*px - ax*py
        // B * (ay*bx - ax*by) = ay*px - ax*py
        // B = (ay*px - ax*py) / (ay*bx - ax*by)
        // by*ax*A + by*bx*B = by*px
        // bx*ay*A + bx*by*B = bx*py
        // by*ax*A - bx*ay*A = by*px - bx*py
        // A * (by*ax - bx*ay) = by*px - bx*py
        // A = (by*px - bx*py) / (by*ax - bx*ay)
        let ax = *block.get(0).unwrap();
        let ay = *block.get(1).unwrap();
        let bx = *block.get(2).unwrap();
        let by = *block.get(3).unwrap();
        let px = *block.get(4).unwrap();
        let py = *block.get(5).unwrap();
        if (ay*px - ax*py) % (ay*bx - ax*by) != 0 || (by*px - bx*py) % (by*ax - bx*ay) != 0 {
            continue;
        }
        let a = ((by*px - bx*py) / (by*ax - bx*ay)).abs();
        let b = ((ay*px - ax*py) / (ay*bx - ax*by)).abs();
        result += 3*a + b;
    }
    println!("{result}");
}

pub fn main() {
    let input = crate::grab_input("day13");
    let extractor= Regex::new(r#"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)"#).unwrap();
    let mut data_blocks = Vec::<Vec<isize>>::new();
    for capture in extractor.captures_iter(&input) {
        let (_, [ax, ay, bx, by, px, py]) = capture.extract();
        let hold = vec![ax, ay, bx, by, px, py];
        data_blocks.push(hold.iter().map(|s| s.parse::<isize>().unwrap()).collect());
    }

    part1(&data_blocks);
}