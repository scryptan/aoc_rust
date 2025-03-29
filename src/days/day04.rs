use std::array::from_fn;

pub const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

 pub struct Input {
    turn: usize,
    score: usize,
}

pub fn parse(input: &str) -> Vec<Input> {
    let mut to_turn = [0; 100];
    let mut from_turn = [0; 100];

    let mut chunks = input.split("\n\n");

    for (i, n) in chunks.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).enumerate() {
        to_turn[n] = i;
        from_turn[i] = n;
    }

    let score = |chunk: &str| {
        let mut iter = chunk.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let board: [usize; 25] = from_fn(|_| iter.next().unwrap());
        let turns: [usize; 25] = from_fn(|i| to_turn[board[i]]);

        let row_and_cols = [
            turns[0..5].iter().max().unwrap(),
            turns[5..10].iter().max().unwrap(),
            turns[10..15].iter().max().unwrap(),
            turns[15..20].iter().max().unwrap(),
            turns[20..25].iter().max().unwrap(),
            turns.iter().step_by(5).max().unwrap(),
            turns.iter().skip(1).step_by(5).max().unwrap(),
            turns.iter().skip(2).step_by(5).max().unwrap(),
            turns.iter().skip(3).step_by(5).max().unwrap(),
            turns.iter().skip(4).step_by(5).max().unwrap(),
        ];
        let winning_turn = **row_and_cols.iter().min().unwrap();
        let unmarked: usize = board.iter().filter(|&&n| to_turn[n] > winning_turn).sum();
        let just_called = from_turn[winning_turn];

        Input { turn: winning_turn, score: unmarked * just_called }
    };

    let mut scores: Vec<_> = chunks.map(score).collect();
    scores.sort_unstable_by_key(|s| s.turn);
    scores
}

pub fn part1(input: &[Input]) -> usize {
    input.first().unwrap().score
}

pub fn part2(input: &[Input]) -> usize {
    input.last().unwrap().score
}

#[cfg(test)]
mod tests {
    use crate::days::day04::*;

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 1924);
    }
}