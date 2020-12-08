/* auth : Trenton Stiles
 * name : aoc1-2
 * desc : solves advent of code 2020 problem 1 part 2
 */

use std::fs;

fn main(){
    let filename = "C:\\Users\\user\\Desktop\\AoC\\1\\src\\input";
    let mut file_contents = fs::read_to_string(filename)
        .expect("Cannot read file.");

    file_contents = file_contents.trim().to_string();
    
    let split: Vec<&str> = file_contents.split("\n").collect();

    let mut z = 0; // tracks index k
    for i in 0..split.len(){
        let first_val: u32 = split[i].parse()
            .expect("cannot parse string.");

        for j in 0..split.len(){
            // if on same index as any other iterator skip
            if j == i || j == z {
                continue;
            }
            let second_val: u32 = split[j].parse()
                .expect("cannot parse string.");     
                
            for k in 0..split.len(){
                z = k;
                // if on same index as any other iterator skip

                if k == i || k == j {
                    continue;
                }
                let third_val: u32 = split[k].parse()
                .expect("cannot parse string.");

                if first_val + second_val + third_val == 2020 {
                    println!("{} + {} + {} = {}",
                        first_val, second_val, third_val,
                        first_val + second_val + third_val);
                    println!("{}", first_val*second_val*third_val);
                    
                }
            }
        }
    }
}