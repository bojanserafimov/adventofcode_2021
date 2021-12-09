use std::collections::HashSet;
use single_item::SingleItemExt;

fn main() {
    let lines: Vec<String> = sport::read_lines_buffered("input.txt").collect();
    let line_length = lines
        .iter()
        .map(|line| line.len())
        .collect::<HashSet<_>>()
        .get_only_item()
        .unwrap();
    let bit_matrix: Vec<Vec<usize>> = lines.iter().map(|line| {
        line.chars().map(|c| c.to_digit(2).unwrap() as usize).collect()
    }).collect();

    let gamma_string: String = (0..line_length)
        .map(|col_idx| bit_matrix.iter().map(|row| row[col_idx]).sum())
        .map(|num_ones: usize| 2 * num_ones >= lines.len())
        .map(|digit| char::from_digit(digit as u32, 2).unwrap())
        .collect();

    let part1 = {
        let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();
        let epsilon = gamma ^ ((1 << line_length) - 1);
        gamma * epsilon
    };
    dbg!(part1);

    let eliminate_iteratively = |use_majority: bool| {
        let mut candidate_rows = bit_matrix.clone();
        for col_idx in 0..line_length {
            if candidate_rows.len() == 1 {
                break;
            }
            let col_sum: usize = candidate_rows.iter().map(|row| row[col_idx]).sum();
            let majority_digit = 2 * col_sum >= candidate_rows.len();
            let desired_digit = majority_digit == use_majority;
            candidate_rows.retain(|row| row[col_idx] == desired_digit as usize);
        }
        let result_string: String = candidate_rows
            .get_only_item()
            .unwrap()
            .iter()
            .map(|&digit| char::from_digit(digit as u32, 2).unwrap())
            .collect();
        u32::from_str_radix(&result_string, 2).unwrap()
    };

    let part2 = {
        let oxygen = eliminate_iteratively(true);
        let co2 = eliminate_iteratively(false);
        oxygen * co2
    };
    dbg!(part2);
}
