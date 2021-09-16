use std::io::prelude::*;
use std::cmp;

fn main() {

    // Handle input
    let input = std::io::stdin();
    let mut lines = input
        .lock()
        .lines()
        .map(|line| line.ok().unwrap());

    // Get dimensions of the desired rectangle, first and only row
    let dimensions = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // extract row and dimensions as variables
    let rows = dimensions[0] + 1;
    let cols = dimensions[1] + 1;
    
    // Loop through ROWS x COLS times
    for row in 1..dimensions[0] + 1 {
        for col in 1..dimensions[1] + 1 {

            /*
                There are 2 horizontal and 2 vertical edges. The value on the current coordinate is the closest distance to the edge.
                Thus to find the value we have to find the closest distance vertically and horizontally.
                Then using that determine if the value is closest on the horizontal side or the vertical side.
            */
            let val = cmp::min(
                cmp::min(row, rows - row),
                cmp::min(col, cols - col)
            );
            
            // Greater than 9 outputs a '.'
            if val <= 9 {
                print!("{}", val);
            }
            else {
                print!(".");
            }
        }
        println!(); // newline
    }
}
