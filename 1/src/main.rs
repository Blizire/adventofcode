/* auth : Trenton Stiles
 * name : aoc1
 * desc : solves advent of code 2020 problem 1
 */

use std::fs;

fn main(){
    let filename = "C:\\Users\\user\\Desktop\\AoC\\1\\src\\input";
    let file_contents = fs::read_to_string(filename)
        .expect("Cannot read file.");

    let mut split: Vec<&str> = file_contents.split("\n").collect();
    split.pop(); // remove newline @ end of file.

    for (i, line) in split.iter().enumerate(){
        let x: u32 = line.parse()
            .expect("Cannot parse this string.");
        for target in i..split.len(){
            let y: u32 = split[target].parse()
                .expect("Cannoot parse this string.");
            if (x + y) == 2020 {
                println!("{}", x*y);
            }
        }
    }
}