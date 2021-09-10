struct Solution {}

impl Solution {
    pub fn binary_search(numbers: Vec<i32>, target: i32) -> i32 {
        let mut low: usize = 0;
        let mut high: usize = numbers.len() - 1;

        while low <= high {
            let middle: usize = (low + high) / 2;

            if let Some(current) = numbers.get(middle) {
                if *current == target {
                    return middle as i32;
                }

                if *current > target {
                    high = middle - 1;
                }

                if *current < target {
                    low = middle + 1;
                }
            }
        }

        return -1;
    }
}

#[test]
fn binary_search() {
    let numbers = vec![1, 2, 2, 3, 8, 10, 22];
    let target = 2;

    let result = Solution::binary_search(numbers, target);

    println!("RESULT IS {:?}", result);
}
