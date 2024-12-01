use std::collections::HashMap;

const INPUT: &str = include_str!("../data.txt");

fn puzzle1() {
    let data = INPUT
        .lines()
        .map(|line| {
            line.splitn(2, "   ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let (mut left, mut right): (Vec<usize>, Vec<usize>) =
        data.into_iter().map(|r| (r[0], r[1])).unzip();

    left.sort();
    right.sort();

    let diff: usize = std::iter::zip(left, right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    // assert_eq!(diff, 1341714);
    println!("[puzzle1] Diff: {diff}");
}

fn puzzle2() {
    let data = INPUT
        .lines()
        .map(|line| {
            line.splitn(2, "   ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let (left, right): (Vec<usize>, Vec<usize>) = data.into_iter().map(|r| (r[0], r[1])).unzip();

    let mut occurrence: HashMap<usize, usize> = HashMap::new();

    right.iter().for_each(|item| {
        occurrence.entry(*item).and_modify(|e| *e += 1).or_insert(1);
    });

    let similarity: usize = left
        .iter()
        .map(|item| {
            if let Some(count) = occurrence.get(item) {
                item * count
            } else {
                0
            }
        })
        .sum();

    // assert_eq!(similarity, 27384707);
    println!("[puzzle2] Similarity: {similarity}")
}

fn main() {
    puzzle1();
    puzzle2();
}
