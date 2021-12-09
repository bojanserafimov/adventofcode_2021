use single_item::SingleItemExt;

fn main() {
    let mut count = vec![0; 9];
    for token in sport::read_lines_buffered("input.txt")
        .get_only_item()
        .unwrap()
        .split(",") {
            let time: usize = token.parse().unwrap();
            count[time] += 1;
        }

    let simulate = |days| {
        let mut count = count.clone();
        for _ in 0..days {
            let mut new_count = vec![0; 9];
            new_count[6] += count[0];
            new_count[8] += count[0];
            for i in 1..=8 {
                new_count[i - 1] += count[i];
            }
            count = new_count;
        }
        count.iter().sum()
    };

    let part1: usize = simulate(80);
    dbg!(part1);

    let part2: usize = simulate(256);
    dbg!(part2);
}
