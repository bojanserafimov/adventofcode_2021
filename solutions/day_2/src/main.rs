fn main() {
    let commands: Vec<(String, u64)> = sport::read_lines_buffered("input.txt")
        .map(|line| {
            let (first, second) = line.split_once(" ").unwrap();
            (String::from(first), second.parse().unwrap())
        })
        .collect();

    let part1 = {
        let mut distance = 0;
        let mut depth = 0;
        for (command, value) in &commands {
            match command.as_str() {
                "forward" => distance += value,
                "up" => depth -= value,
                "down" => depth += value,
                _ => unreachable!()
            }
        }
        distance * depth
    };
    dbg!(part1);

    let part2 = {
        let mut distance = 0;
        let mut depth = 0;
        let mut aim = 0;
        for (command, value) in &commands {
            match command.as_str() {
                "forward" => {
                    distance += value;
                    depth += value * aim;
                },
                "up" => aim -= value,
                "down" => aim += value,
                _ => unreachable!()
            }
        }
        distance * depth
    };
    dbg!(part2);
}
