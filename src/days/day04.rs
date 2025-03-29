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


 #[derive(Debug)]
pub struct Input{
    numbers: Vec<u32>,
    boards: Vec<Board>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board{
    matrix: Vec<Vec<u32>>,
    marked: Vec<u32>,
    is_winner: bool,
}

pub fn parse(input: &str) -> Input{
    let lines = input.lines().next().expect("should contain at least 1 line").split(',').map(|x| x.parse::<u32>().expect("Should be int")).collect();

    let boards = get_boards(input);
     
    Input { numbers: lines, boards }
}

fn get_boards(input: &str) -> Vec<Board> {
    let mut boards = Vec::new();
    let mut board = Vec::new();
    let marked = Vec::new();

    for line in input.lines().skip(2){
        if line.is_empty(){
            boards.push(Board { matrix: board.clone(), marked: marked.clone(), is_winner: false });
            board.clear();
        }else{
            let row = line.split_whitespace().map(|x| x.parse::<u32>().expect("Should be int")).collect::<Vec<u32>>();
            board.push(row);
        }
    }

    boards.push(Board{matrix: board, marked, is_winner: false});

    boards
}

fn check_winner(board: &Board) -> Option<Vec<u32>> {
    // Check rows
    for row in &board.matrix {
        if row.iter().all(|&x| board.marked.contains(&x)) {
            return Some(row.clone());
        }
    }

    // Check columns
    for i in 0..board.matrix[0].len() {
        if board.matrix.iter().all(|row| board.marked.contains(&row[i])) {
            return Some(board.matrix.iter().map(|row| row[i]).collect::<Vec<u32>>());
        }
    }

    None
}

fn get_all_unmarked(board: &Board) -> Vec<u32> {
    let mut unmarked = Vec::new();
    for row in &board.matrix {
        for &num in row {
            if !board.marked.contains(&num) {
                unmarked.push(num);
            }
        }
    }
    unmarked
}

pub fn part1(input: &Input) -> u32{
    let mut boards = input.boards.clone();
    let mut numbers = input.numbers.clone();
    let mut winner = false;
    let mut score = 0;

    for number in numbers {
        // println!("Number: {}", number);
        for board in &mut boards {
            // println!("Marked: {:?}", board.marked);
            if !board.marked.contains(&number) {
                board.marked.push(number);
            }
            let winner_check = check_winner(board);
            if winner_check.is_some() {
                // println!("Winner board: {:?}", board);
                winner = true;
                let sum = get_all_unmarked(board).iter().sum::<u32>();
                score = number  * sum;
                // println!("number: {number}, sum: {sum}, {:?}", winner_check.unwrap());
                break;
            }
        }
        if winner {
            break;
        }
    }

    score
}

pub fn part2(input: &Input) -> u32{
    let mut boards = input.boards.clone();
    let mut winner_boards = Vec::new();
    let numbers = input.numbers.clone();
    let boards_count = boards.len();
    let mut winner = false;
    let mut score = 0;

    for number in numbers {
        // println!("Number: {}", number);
        

        if winner {
            let board = boards.iter_mut().find(|b| !b.is_winner).unwrap(); // Find the last winning board
            board.marked.push(number);
            println!("Winner board: {:?}", board);
            // println!("Winner boards: {:?}", winner_boards);

            let sum = get_all_unmarked(board).iter().sum::<u32>();
            println!("number: {number}, sum: {sum}, {:?}", board.marked);
            score = number  * sum;
            break;
        }

        for board in &mut boards {
            if board.is_winner {
                continue; // Skip already winning boards
            }
            // println!("Marked: {:?}", board.marked);
            if !board.marked.contains(&number) {
                board.marked.push(number);
            }

            let winner_check = check_winner(board);
            if winner_check.is_some() {
                board.is_winner = true; // Mark the board as a winner
                winner_boards.push(board.clone()); // Add the winning board to the list

                if boards_count - 1 == winner_boards.len() {
                    winner = true;
                }
            }
        }
    }

    score
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