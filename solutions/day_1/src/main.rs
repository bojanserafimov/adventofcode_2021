
fn main() {
    let depths: Vec<u64> = sport::read_lines_buffered("input.txt")
        .map(|line| line.parse().unwrap())
        .collect();

    let part1 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(prev, curr)| prev < curr)
        .count();
    dbg!(part1);

    let part2 = depths
        .iter()
        .zip(depths.iter().skip(3))
        .filter(|(prev, curr)| prev < curr)
        .count();
    dbg!(part2);
}
