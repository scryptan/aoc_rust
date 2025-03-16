pub const EXAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263";

pub fn parse(input: &str) -> Vec<u32>{
    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}


pub fn part1(input: &Vec<u32>) -> i32{
    let mut current_value :u32 = input[0];
    let mut count = 0;

    for i in input{
        if *i > current_value{
            count += 1;
        }
        current_value = *i;
    }

    count
}

pub fn part2(input: &Vec<u32>) -> i32{
    let mut current_value :u32 = input[0] + input[1] + input[2];
    let mut count = 0;

    for i in 0..input.len() - 2{
        let curr_sum = input[i] + input[i + 1] + input[i + 2];
        if curr_sum > current_value{
            count += 1;
        }
        current_value = curr_sum;
    }

    count
}

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 7);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 5);
}