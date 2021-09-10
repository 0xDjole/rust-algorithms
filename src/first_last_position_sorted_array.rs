struct Solution {}

impl Solution {
    pub fn first_last_position_sorted_array(numbers: Vec<i32>, target: i32) -> [i32; 2] {
        let left_position = Self::binary_search(&numbers, target, true);
        let right_position = Self::binary_search(&numbers, target, false);

        [left_position, right_position]
    }
    fn binary_search(numbers: &Vec<i32>, target: i32, is_left: bool) -> i32 {
        let mut index = -1;
        let mut low: usize = 0;
        let mut high: usize = numbers.len() - 1;

        while low <= high {
            let middle: usize = (low + high) / 2;
            println!("MIDDLE {:?}", middle);
            if let Some(current) = numbers.get(middle) {
                if *current == target {
                    index = middle as i32;
                }

                if (is_left && *current >= target) || (!is_left && *current > target) {
                    high = middle - 1;
                }

                if (is_left && *current < target) || (!is_left && *current <= target) {
                    low = middle + 1;
                }
            }
        }

        return index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_last_position_sorted_array() {
        let numbers = vec![1, 3, 3, 3, 22];
        let target = 3;

        let result = Solution::first_last_position_sorted_array(numbers, target);

        println!("RESULT IS {:?}", result);
    }
}
