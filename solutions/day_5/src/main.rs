use std::{collections::HashMap, ops::AddAssign};

fn main() {
    let lines: Vec<((i64, i64), (i64, i64))> = sport::read_lines_buffered("input.txt")
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            let parse_point = |s: &str| {
                let (x, y) = s.split_once(",").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            };
            (parse_point(a), parse_point(b))
        })
        .collect();

    let sign = |x| {
        if x < 0 {
            return -1
        } else if x == 0 {
            return 0
        } else {
            return 1
        }
    };
    let walk = |line: ((i64, i64), (i64, i64))| std::iter::successors(Some(line.0), move |prev| {
        if prev.0 == line.1.0 && prev.1 == line.1.1 {
            return None
        }
        return Some((
            prev.0 + sign(line.1.0 - line.0.0),
            prev.1 + sign(line.1.1 - line.0.1)
        ))
    });
    let solve = |include_diagonal| {
        let mut map: HashMap<(i64, i64), usize> = HashMap::new();
        for &line in &lines {
            if include_diagonal || line.0.0 == line.1.0 || line.0.1 == line.1.1 {
                for point in walk(line) {
                    map.entry(point).or_insert(0).add_assign(1);
                }
            }
        }
        map.iter()
            .filter(|(_, &count)| count >= 2)
            .count()
    };

    let part1 = solve(false);
    dbg!(part1);

    let part2 = solve(true);
    dbg!(part2);
}
