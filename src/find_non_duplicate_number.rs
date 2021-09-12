struct Solution {}

impl Solution {
    fn find_non_duplicate_number(numbers: Vec<i32>) -> i32 {
        let mut result = 0;

        for number in numbers {
            result ^= number;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_non_duplicate_number() {
        let numbers = vec![5, 5, 4, 4, 1, 1, 3];
        let result = Solution::find_non_duplicate_number(numbers);
        println!("RESULT IS {:?} ", result);
    }
}
