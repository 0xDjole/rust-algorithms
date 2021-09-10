use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, wanted_sum: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (index, number) in numbers.iter().enumerate() {
            let wanted_seen_number = wanted_sum - number;

            if let Some(exists) = seen.get(&wanted_seen_number) {
                return vec![*exists, index as i32];
            } else {
                seen.insert(*number, index as i32);
            }
        }

        vec![]
    }
}

#[test]
fn two_sum() {
    let numbers = vec![1, 2, 3, 4, 5];
    let wanted_sum = 7;
    let result = Solution::two_sum(numbers, wanted_sum);

    println!("Solution is {:?}", result);
}
