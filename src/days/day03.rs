pub const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

pub struct Input<'a>{
    lines: Vec<&'a[u8]>,
    width: usize
}

pub fn parse(input: &str) -> Input{
    let lines = input.lines().map(|x| x.as_bytes()).collect();
    let len = input.lines().next().expect("no line").as_bytes().len();
    Input { lines, width: len }
}


pub fn part1(input: &Input<'_>) -> u32{
    let mut gamma_binary :Vec<u32> = Vec::new();
    let mut epsilon_binary :Vec<u32> = Vec::new();

    for i in 0..input.width{
        let mut count :u32 = 0;
        for line in &input.lines{
            let value = line[i];
            let a = value - 48;
            count += a as u32;
        }

        if count > (input.lines.len() / 2).try_into().expect("can't convers"){
            gamma_binary.push(1);
            epsilon_binary.push(0);
        }else{
            gamma_binary.push(0);
            epsilon_binary.push(1);
        }
    }
    let gamma = get_value(&gamma_binary);
    let epsilon = get_value(&epsilon_binary);

    gamma * epsilon
}

pub fn part2(input: &Input<'_>) -> u32{
    let gamma = take_binary(&input.lines, input.width, true);
    let epsilon = take_binary(&input.lines, input.width,false);

    gamma * epsilon
}

fn take_binary(bytes: &Vec<&[u8]>, width: usize, gamma: bool) -> u32{
    let mut temp = bytes.clone();

    for i in 0..width{
        print_values(&temp);
        let mut count :i32 = 0;
        let len: i32 = temp.len().clone() as i32;
        if len == 1{
            break;
        }
        for line in &temp {
            let value = line[i];
            let a = value - 48;
            count += a as i32;
        }
        // println!("{len}|{count}");
        
        let a :i32 = len - count;

        if gamma {
            if len == 2 {
                temp = temp.iter().filter(|x| x[i] == 49).map(|x| *x).collect();
            }else if count >= a{
                temp = temp.iter().filter(|x| x[i] == 49).map(|x| *x).collect();
            }else{
                temp = temp.iter().filter(|x| x[i] == 48).map(|x| *x).collect();            
            }
        }else{
            if len == 2 {
                temp = temp.iter().filter(|x| x[i] == 48).map(|x| *x).collect();
            }else if count > a{
                temp = temp.iter().filter(|x| x[i] == 48).map(|x| *x).collect();
            }else{
                temp = temp.iter().filter(|x| x[i] == 49).map(|x| *x).collect();            
            }
        }
    }


    // print_values(&temp);
    let res = &temp[0].iter().map(|x|(*x as u32) - 48).collect::<Vec<u32>>();
    return get_value(res);
}

fn print_values(bytes: &Vec<&[u8]>){
    for line in bytes{
        for i in *line{
            let value = i - 48;
            print!("{value} ");
        }
        println!()
    }
}

fn get_value(bytes: &Vec<u32>) -> u32{
    let mut res = 0;
    let len = bytes.len();
    for i in 0..len{
        res += 2_u32.pow(i.try_into().expect("can't usize to u32")) * bytes[len - i - 1];
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::days::day03::*;

    #[test]
    fn part1_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn part2_test() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 230);
    }
}