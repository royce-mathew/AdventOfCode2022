use std::{fs};

struct Solution;

impl Solution {
    fn part1(x: &str) -> u32 {
        let mut assignment_pairs: u32 = 0;
        let read_str = fs::read_to_string(x).unwrap_or("unwrap err".to_owned());

        for line in read_str.split_whitespace() {

            // Convert the line to a vector of integers
            let split_vec: Vec<u32> = line
            .split([',', '-'])
            .into_iter()
            .map(|x| x
                .parse::<u32>()
                .unwrap()
            )
            .collect();


            /*
                The incrementing of an assignment pair can only occur on three situations.
                    When the first array is inside the bound of the second array
                    When the second array is inside the bound of the first array
                    Also, when both arrays iterators have the same starting index, 
                        we can conclude that they are both inevitably going to be in one of the other's bounds
            */
            if split_vec[1] <= split_vec[3] && split_vec[0] >= split_vec[2] {
                assignment_pairs += 1;
            } else if split_vec[1] >= split_vec[3] && split_vec[0] <= split_vec[2] {
                assignment_pairs += 1;
            } else if split_vec[0] == split_vec[2] {
                assignment_pairs += 1;
            }
        }

        assignment_pairs
    }


    // 823
    fn part2(x: &str) -> u32 {
        let mut assignment_pairs: u32 = 0;
        let read_str = fs::read_to_string(x).unwrap_or("unwrap err".to_owned());

        for line in read_str.split_whitespace() {

            // Convert the line to a vector of integers
            let split_vec: Vec<u32> = line
            .split([',', '-'])
            .into_iter()
            .map(|x| x
                .parse::<u32>()
                .unwrap()
            )
            .collect();

            

            /*
                The solution here was to use a hashset to calculate whether the array had a duplicate value inside

            // */

            // All four values are same
            if split_vec[0] == split_vec[1] && split_vec[0] == split_vec[2] && split_vec[0] == split_vec[3] {
                assignment_pairs += 1;
                continue;
            }

            
            // Check if first array is inside second array
            if split_vec[1] <= split_vec[3] && split_vec[0] >= split_vec[2] {
                assignment_pairs += 1;
                continue;
            } 
            
            // Check if second array is inside first array
            if split_vec[1] >= split_vec[3] && split_vec[0] <= split_vec[2] {
                assignment_pairs += 1;
                continue;
            } 
            
            // Check if first two values and one values in second array are equal, 3 3 3 1
            if split_vec[0] == split_vec[1] && (split_vec[0] <= split_vec[3] && split_vec[0] >= split_vec[2])  {
                assignment_pairs += 1;
                continue;
            }
            
            // Check if values in second array are equal to one value in the first array
            if split_vec[2] == split_vec[3] && (split_vec[2] <= split_vec[1] && split_vec[2] <= split_vec[0]) {
                assignment_pairs += 1;
                continue;
            }


            // if any range indexes touch
            if split_vec[0] == split_vec[3] || split_vec[1] == split_vec[2] {
                assignment_pairs += 1;
                continue;
            }

            println!("{:?}", split_vec);

        }

        assignment_pairs
    }


    fn otherPart1(x: &str) -> u32 {
        let mut assignment_pairs: u32 = 0;
        let read_str = fs::read_to_string(x).unwrap_or("unwrap err".to_owned());

        for line in read_str.split_whitespace() {

            // Convert the line to a vector of integers
            let split_vec: Vec<u32> = line
            .split([',', '-'])
            .into_iter()
            .map(|x| x
                .parse::<u32>()
                .unwrap()
            )
            .collect();


            let pair1 = split_vec[0]..=split_vec[1];
            let pair2 = split_vec[2]..=split_vec[3];

            if pair1.contains(&(split_vec[2])) && pair1.contains(&(split_vec[3])) {
                assignment_pairs += 1;
                continue;
            }

            if pair2.contains(&(split_vec[0])) && pair2.contains(&(split_vec[1])) {
                assignment_pairs += 1;
                continue;
            }

        }

        assignment_pairs
    }


    fn otherPart2(x: &str) -> u32 {
        let mut assignment_pairs: u32 = 0;
        let read_str = fs::read_to_string(x).unwrap_or("unwrap err".to_owned());

        for line in read_str.split_whitespace() {

            // Convert the line to a vector of integers
            let split_vec: Vec<u32> = line
                .split([',', '-'])
                    .into_iter()
                        .map(|x| x
                            .parse::<u32>()
                            .unwrap()
                        )
                        .collect();

            let pair1 = split_vec[0]..=split_vec[1];
            let pair2 = split_vec[2]..=split_vec[3];

            if pair1.start() <= pair2.end() && pair1.end() >= pair2.start() {
                assignment_pairs += 1;
            }


        }

        assignment_pairs
    }
}

fn main() {
    let j = Solution::part2("src/data.txt");
    let x = Solution::otherPart2("src/data.txt");
    
    println!("{}", j);
    println!("{}", x);
}
