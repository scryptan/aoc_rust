pub const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

pub fn parse(input: &str) -> Vec<&str>{
    input.lines().collect()
}


pub fn part1(input: &Vec<&str>) -> i32{
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input{
        let sp :Vec<&str> = line.split_ascii_whitespace().collect();
        // print!("{:?}", sp);
        let command = sp[0];
        let value = sp[1].parse::<i32>().expect("can't parse value");

        match command {
            "forward" => horizontal += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => panic!("Don't known command {}", command)
        }
    }

    horizontal * depth
}

pub fn part2(input: &Vec<&str>) -> i32{
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input{
        let sp :Vec<&str> = line.split_ascii_whitespace().collect();
        // print!("{:?}", sp);
        let command = sp[0];
        let value = sp[1].parse::<i32>().expect("can't parse value");

        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            },
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("Don't known command {}", command)
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use crate::days::day02::*;

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 900);
    }
}