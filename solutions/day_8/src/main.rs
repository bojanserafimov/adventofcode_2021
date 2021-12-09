use std::io::BufRead;
use std::collections::HashMap;
use std::ops::AddAssign;

use single_item::SingleItemExt;

fn main() {
    let lines: Vec<(Vec<String>, Vec<String>)> = sport::read_lines_buffered("input.txt")
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            (left.split(" ").map(|s| s.to_string()).collect(),
             right.split(" ").map(|s| s.to_string()).collect())
        })
        .collect();

    let part1: usize = lines
        .iter()
        .map(|line|
             line.1.iter().filter(|w| [2, 3, 4, 7].contains(&w.len())).count())
        .sum();
    dbg!(part1);

    let part2: usize = lines
        .iter()
        .map(|line| {
            let one = line.0.iter().filter(|w| w.len() == 2).get_only_item().unwrap();
            let four = line.0.iter().filter(|w| w.len() == 4).get_only_item().unwrap();
            let recognize = |w: String| {
                let intersect_4 = w.chars().filter(|c| four.contains(&c.to_string())).count();
                let intersect_1 = w.chars().filter(|c| one.contains(&c.to_string())).count();
                match (w.len(), intersect_4, intersect_1) {
                    (2, _, _) => 1,
                    (3, _, _) => 7,
                    (4, _, _) => 4,
                    (7, _, _) => 8,
                    (5, 2, _) => 2,
                    (5, 3, 2) => 3,
                    (5, 3, 1) => 5,
                    (6, 4, _) => 9,
                    (6, 3, 2) => 0,
                    (6, 3, 1) => 6,
                    _ => unreachable!()
                }
            };
            let recognized: String = line.1.iter()
                .map(|w| char::from_digit(recognize(w.clone()), 10).unwrap())
                .collect();
            usize::from_str_radix(&recognized, 10).unwrap()
        })
        .sum();
    dbg!(part2);
}
