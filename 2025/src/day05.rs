

fn part1(fresh_ranges: &Vec<(u64, u64)>, ingredient_ids: &Vec<u64>) {
    let valid_ids = ingredient_ids.iter()
        .filter(|&ingredient_id| 
            fresh_ranges.iter().any(|(range_start, range_end)| 
                range_start <= ingredient_id && ingredient_id <= range_end
            )
        )
        .count();

    println!("{}", valid_ids);
}

fn part2(fresh_ranges: &Vec<(u64, u64)>) {
    let mut headaches: Vec<(u64,u64,bool)> = Vec::new(); // start, end, ordinality (add/subtract)
    for (new_start,new_end) in fresh_ranges {
        let mut new_headaches = Vec::new();
        for (old_start, old_end, old_add) in &headaches {
            // mask some variables so we have left range and right range rather than new/old
            // go by start distance for left vs right, if same start then whichever ENDS SOONER is left
            let (_left_start, left_end, right_start, right_end) = 
                if (new_start < old_start) || (new_start == old_start && new_end < old_end) {
                    (new_start, new_end, old_start, old_end)
                } else {
                    (old_start, old_end, new_start, new_end)
                };
            
            // check for NO OVERLAP AT ALL (best case please be this case please)
            if left_end < right_start {continue;}
            
            // begrudgingly, we must handle, overlap
            if right_end < left_end { // total overlap
                new_headaches.push((*right_start, *right_end, !old_add));
            } else { // partial overlap
                new_headaches.push((*right_start, *left_end, !old_add));
            }
        }
        headaches.extend(new_headaches);
        headaches.push((*new_start, *new_end, true));
    }

    let result: i64 = headaches.iter().map(|(start,end,add)| (end - start + 1) as i64 * if *add {1} else {-1}).sum();
    println!("{}", result);
}

fn refactored_main(mut fresh_ranges: Vec<(u64, u64)>, ingredient_ids: Vec<u64>) {
    fresh_ranges.sort();
    let mut compacted_ranges: Vec<(u64, u64)> = Vec::new();
    let (mut compacted_start, mut compacted_end) = fresh_ranges.get(0).unwrap().clone();
    for (fresh_start, fresh_end) in fresh_ranges {
        if fresh_start < compacted_end { // extends current window
            compacted_end = if compacted_end > fresh_end {compacted_end} else {fresh_end};
        } else { // start of a new window
            compacted_ranges.push((compacted_start, compacted_end));
            compacted_start = fresh_start;
            compacted_end = fresh_end;
        }
    }
    compacted_ranges.push((compacted_start, compacted_end));

    use std::time::Instant;
    let mut now = Instant::now();
    {part1(&compacted_ranges, &ingredient_ids);}
    let mut elapsed = now.elapsed();
    println!("Part 1 refactored: {:.2?}", elapsed);
    
    now = Instant::now();
    {part2(&compacted_ranges);}
    elapsed = now.elapsed();
    println!("Part 2 refactored: {:.2?}", elapsed);
}

pub fn main() {
    let input = crate::grab_input("day05");
    let (fresh_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<(u64, u64)> = fresh_ranges.split_whitespace()
        .map(|row| {
            let (a,b) = row.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }).collect();

    let ingredient_ids: Vec<u64> = ingredient_ids.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    
    use std::time::Instant;
    let full_now = Instant::now();
    let mut now = Instant::now();
    {part1(&fresh_ranges, &ingredient_ids);}
    let mut elapsed = now.elapsed();
    println!("Part 1 normal: {:.2?}", elapsed);
    
    now = Instant::now();
    {part2(&fresh_ranges);}
    elapsed = now.elapsed();
    println!("Part 2 normal: {:.2?}", elapsed);
    elapsed = full_now.elapsed();
    println!("Full normal: {:.2?}", elapsed);
    
    now = Instant::now();
    {refactored_main(fresh_ranges, ingredient_ids);}
    elapsed = now.elapsed();
    println!("Full refactored: {:.2?}", elapsed);
}