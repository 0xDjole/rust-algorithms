use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn queue_reconstruction(mut queue: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
        queue.sort_by(|a, b| {
            if a[0] > b[0] {
                return a[0].partial_cmp(&b[0]).map(Ordering::reverse).unwrap();
            } else {
                return Ordering::Equal;
            }
        });
        println!("SORTED {:?}", queue);
        let mut result = vec![];
        for person in queue {
            result.push(person);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_reconstruction() {
        // [[5, 0] [7, 0] [5, 2] [6, 1]]
        let queue = vec![[7, 0], [5, 0], [5, 2], [6, 1]];
        let result = Solution::queue_reconstruction(queue);
        println!("RESULT IS {:?} ", result);
    }
}
