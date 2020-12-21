/* auth : Trenton Stiles
 * name : aoc2-1
 * desc : solves advent of code 2020 problem 2 part 2
 */

use std::fs;

fn main() {
    // read text file
    let fpath = "C:\\Users\\user\\Desktop\\adventofcode\\2-1\\src\\input";
    // load text from file to variable
    let rawdata = fs::read_to_string(fpath)
        .expect("Failed to read input file");

    // is <Lines> but acts like an array of lines from text file
    let lines = rawdata.lines();

    // total valid passwords at the end
    let mut valid_pws = 0;

    // loop through each lines of the file
    for line in lines {
        let policy   = match line.split(":").nth(0) {
            None => "could not get policy",
            Some(x) => x,
        };

        let password = match line.split(": ").nth(1) {
            None => "could not get password",
            Some(x) => x,
        };

        let (policy_pos_one,
             policy_pos_two,
             policy_req) = get_policy_pos(policy);
        
        let (letter_pos_one, letter_pos_two) = get_pwchars(
            policy_pos_one,
            policy_pos_two,
            password);
        
        if letter_pos_one == policy_req
            && letter_pos_two != policy_req {
            valid_pws += 1;
        } 
        else if letter_pos_one != policy_req
            && letter_pos_two == policy_req {
            valid_pws += 1;
        }
    } // end of for loop
    println!("valid pws : {}", valid_pws);
}

fn get_policy_pos(policy: &str) -> (&str, &str, char){
    let tmp     = policy.split("-").collect::<Vec<&str>>();
    let tmp2    = tmp[1].split(" ").collect::<Vec<&str>>();

    let pos_one = tmp[0];
    let pos_two = tmp2[0];
    let req     = tmp2[1].chars().collect::<Vec<char>>()[0];
    return (pos_one, pos_two, req);
}

fn get_pwchars<'a>(pos1: &str, pos2: &str, pw: &'a str) -> (char, char) {
    let pos1 = match pos1.parse::<usize>() {
        Ok(x) => x - 1,
        Err(_x) => {
            0
        },
    };

    let pos2 = match pos2.parse::<usize>() {
        Ok(x) => x - 1, // +1 because pw policy index starts at 1 not 0
        Err(_x) => {
            0
        },
    };

    let x = match pw.chars().nth(pos1) {
        Some(x) => x,
        None => {
            '\0'
        }
    };
    let y = match pw.chars().nth(pos2) {
        Some(x) => x,
        None => {
            '\0'
        }
    };

    return (x, y);
}