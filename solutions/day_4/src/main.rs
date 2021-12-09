use std::collections::HashSet;
use single_item::SingleItemExt;


fn get_score(numbers: &Vec<u64>, board: &Vec<Vec<u64>>) -> (usize, u64) {
    let mut marked = vec![vec![false; 5]; 5];
    for (idx, &number) in numbers.iter().enumerate() {
        for row in 0..5 {
            for col in 0..5 {
                if board[row][col] == number {
                    marked[row][col] = true;

                    let full_row = (0..5).all(|i| marked[row][i]);
                    let full_col = (0..5).all(|i| marked[i][col]);
                    if full_row || full_col {
                        let mut score = 0;
                        for i in 0..5 {
                            for j in 0..5 {
                                if !marked[i][j] {
                                    score += board[i][j];
                                }
                            }
                        }
                        score *= number;
                        return (idx, score);
                    }
                }
            }
        }
    }
    (numbers.len(), 0)
}

fn main() {
    let lines: Vec<_> = sport::read_lines_buffered("input.txt").collect();

    let numbers: Vec<u64> = lines[0]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Vec<Vec<u64>>> = lines[1..]
        .chunks(6)
        .map(|chunk|
            chunk.iter().skip(1).map(|line|
                line.split(" ")
                    .filter(|token| !token.is_empty())
                    .map(|token| token.parse().unwrap())
                    .collect()
            ).collect()
        ).collect();

    let (_, part1) = boards
        .iter()
        .map(|board| get_score(&numbers, board))
        .min_by_key(|(time, _)| time.clone())
        .unwrap();
    dbg!(part1);

    let (_, part2) = boards
        .iter()
        .map(|board| get_score(&numbers, board))
        .max_by_key(|(time, _)| time.clone())
        .unwrap();
    dbg!(part2);
}
