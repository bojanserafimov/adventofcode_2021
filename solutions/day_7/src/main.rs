use single_item::SingleItemExt;

fn main() {
    let sorted = {
        let mut numbers: Vec<i64> = sport::read_lines_buffered("input.txt")
            .get_only_item()
            .unwrap()
            .split(",")
            .map(|token| token.parse().unwrap())
            .collect();
        numbers.sort();
        numbers
    };
    let median = sorted[sorted.len() / 2];
    let part1: i64 = sorted.iter().map(|x| (x - median).abs()).sum();
    dbg!(part1);

    let lin_prog = |x| x * (x + 1) / 2;
    let mean_floor: i64 = sorted.iter().sum::<i64>() / sorted.len() as i64;
    let part2 = [mean_floor, mean_floor + 1]
        .iter()
        .map(|center| sorted.iter().map(|x| lin_prog((x - center).abs())).sum::<i64>())
        .min()
        .unwrap();
    dbg!(part2);
}
