/* auth : Trenton Stiles
 * name : aoc2-1
 * desc : solves advent of code 2020 problem 2 part 1
 */

use std::fs;

fn main() {
    let fpath = "C:\\Users\\user\\Desktop\\AoC\\2\\src\\input";
    let rawdata = fs::read_to_string(fpath)
        .expect("Failed to read input file");

    let rawdata = rawdata.trim().split("\n");
    let mut valid_pws = 0;

    for line in rawdata{
        let data = String::from(line);
        let pp_split: Vec<&str> = data
            .split(":").collect(); // policy and password pp_split

        let policy = pp_split[0].trim();
        let password = pp_split[1].trim();

        let policy_min = policy
            .split("-")
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .expect("could not parse policy min");
        let policy_max = policy
            .split("-")
            .collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .expect("could not parse policy max");
        let policy_req = policy
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .chars()
            .next();

        let mut req_occurence = 0;
        for c in password.chars() {
            if policy_req == Some(c) { 
                req_occurence += 1;
            }
        }

        if req_occurence <= policy_max{
            if req_occurence >= policy_min{
                valid_pws += 1;
            }
        }
    }

    println!("{}", valid_pws)
}