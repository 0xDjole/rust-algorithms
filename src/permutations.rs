struct Solution {}

impl Solution {
    pub fn permute(numbers: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute_helper(numbers, 0)
    }

    fn permute_helper(mut numbers: Vec<i32>, start: i32) -> Vec<Vec<i32>> {
        if start == numbers.len() as i32 - 1 {
            return vec![numbers];
        }

        let mut result: Vec<Vec<i32>> = vec![];

        for i in start..numbers.len() as i32 {
            numbers.swap(start as usize, i as usize);
            let batch = Self::permute_helper(numbers.clone(), start + 1);
            result = [result, batch].concat();
            numbers.swap(start as usize, i as usize);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutations() {
        let numbers = vec![1, 2, 3];

        let result = Solution::permute(numbers);

        println!("RESULT IS {:?}", result);
    }
}
