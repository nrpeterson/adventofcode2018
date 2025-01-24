use itertools::{multizip, Itertools};
use nom::Parser;
use adventofcode2018::build_main;

fn power_levels(serial: isize) -> Vec<Vec<isize>> {
    (1..=300).map(|i| {
        (1..=300).map(|j| {
            let rack_id = i + 10;
            let mut power_level = rack_id * j;
            power_level += serial;
            power_level *= rack_id;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            power_level
        }).collect_vec()
    }).collect_vec()
}

fn part1(input: &str) -> String {
    let serial = input.parse::<isize>().unwrap();
    let levels = power_levels(serial);

    let (x, y) = levels.into_iter().tuple_windows().enumerate()
        .flat_map(|(i, (l1, l2, l3))| {
            multizip((l1, l2, l3)).tuple_windows().enumerate()
                .map(move |(j, (t1, t2, t3))| {
                    ((i, j), t1.0 + t1.1 + t1.2 + t2.0 + t2.1 + t2.2 + t3.0 + t3.1 + t3.2)
                })

        })
        .max_by_key(|(_, total)| *total)
        .map(|(level, _)| level)
        .unwrap();

    format!("{},{}", x+1, y+1)
}

fn part2(input: &str) -> String {
    let serial = input.parse::<isize>().unwrap();
    let ref levels = power_levels(serial);

    let mut best = (0, 0, 0);
    let mut best_total = isize::MIN;

    let mut sums = vec![vec![0; 300]; 300];
    sums[0][0] = levels[0][0];

    for i in 1..300 {
        sums[i][0] = sums[i-1][0] + levels[i][0];
        sums[0][i] = sums[0][i-1] + levels[0][i];
    }

    for i in 1..300 {
        for j in 1..300 {
            sums[i][j] = levels[i][j] + sums[i-1][j] + sums[i][j-1] - sums[i-1][j-1];
        }
    }

    for size in 1..=300 {
        for i in 0..301-size {
            for j in 0..301-size {
                let i_end = i + size - 1;
                let j_end = j + size - 1;

                let d = sums[i_end][j_end];
                let a = if i == 0 || j == 0 { 0 } else { sums[i-1][j-1] };
                let b = if i == 0 { 0 } else { sums[i - 1][j_end] };
                let c = if j == 0 { 0 } else { sums[i_end][j - 1] };

                let total = d + a - b - c;
                if total > best_total {
                    best = (i, j, size);
                    best_total = total;
                }
            }
        }
    }

    let (x, y, size) = best;
    format!("{},{},{}", x+1, y+1, size)
}

build_main!("day11.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("18"), "33,45");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("18"), "90,269,16");
    }
}