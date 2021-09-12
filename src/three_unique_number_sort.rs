struct Solution {}

impl Solution {
    fn three_unique_number_sort(mut numbers: Vec<i32>) -> Vec<i32> {
        let mut high = numbers.len() - 1;
        let mut low = 0;
        let mut index = 0;

        while index <= high {
            let number: i32 = numbers[index];

            if number == 1 {
                numbers.swap(index, low);
                low += 1;
                index += 1;
            }

            if number == 2 {
                index += 1;
            }

            if number == 3 {
                numbers.swap(index, high);
                high -= 1;
            }
        }

        numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_unique_number_sort() {
        let numbers = vec![1, 2, 1, 3, 2, 1];
        let result = Solution::three_unique_number_sort(numbers);
        println!("RESULT IS {:?}", result);
    }
}
