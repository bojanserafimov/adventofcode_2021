struct DFS {
    visited: Vec<bool>,
    dir: [i32; 4],
}

impl DFS {
    fn run(&mut self, i: i32) -> i32 {
        match self.visited.get(i as usize) {
            None => 0,
            Some(true) => 0,
            _ => {
                self.visited[i as usize] = true;
                self.dir.clone().iter().map(|di| self.run(i + di)).sum::<i32>() + 1
            }
        }
    }
}

fn main() {
    let grid: Vec<Vec<i32>> = sport::read_lines_buffered("input.txt")
        .map(|line| (line + "9").chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
    let rows = grid.len() as i32;
    let columns = grid.iter().map(|line| line.len()).max().unwrap() as i32;
    let grid: Vec<i32> = grid.into_iter().flatten().collect();
    let dir: [i32; 4] = [1, -1, columns, -columns];

    let part1: i32 = grid.iter().enumerate().filter_map(|(i, h)|
        dir.iter().filter_map(|&di| grid.get((i as i32 + di) as usize)).all(|dh| dh > h).then(|| h + 1)
    ).sum();
    dbg!(part1);

    let part2: i32 = {
        let mut dfs = DFS {visited: grid.iter().map(|&h| h == 9).collect(), dir};
        let mut sizes: Vec<_> = (0..rows * columns).map(|i| dfs.run(i)).collect();
        sizes.sort();
        sizes.iter().rev().take(3).product()
    };
    dbg!(part2);
}
