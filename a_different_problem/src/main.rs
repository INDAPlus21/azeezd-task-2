use std::io::prelude::*;

fn main() {
    
    // Handle input
    let input = std::io::stdin();
    let lines = input
        .lock()
        .lines()
        .map(|line| line.ok().unwrap());

    /* 
        Go through each line, extract the pair and convert it into i64
        Then find the abs value of their difference

        The reason behind i64 is because the problems states the range is [0, 10^15]
        in binary the upper bound is 2^(log_2(10) x 15) which is approximately 2^50 so we need 64 bits
    */
        for line in lines {
            let pairs = line
                .trim()
                .split(' ')
                .map(|a| a.parse().unwrap())
                .collect::<Vec<i64>>();

            let val = (pairs[0] - pairs[1]).abs();

            println!("{}", val);
        }
}
    