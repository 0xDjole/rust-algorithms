use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn queue_reconstruction(mut queue: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
        queue.sort_by(|a, b| {
            if a[0] > b[0] {
                a[0].partial_cmp(&b[0]).map(Ordering::reverse).unwrap()
            } else if a[0] == b[0] {
                a[1].partial_cmp(&b[1]).map(Ordering::reverse).unwrap()
            } else {
                Ordering::Equal
            }
        });

        let mut result = vec![];
        for person in queue {
            result.insert(person[1] as usize, person);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_reconstruction() {
        let queue = vec![[7, 0], [5, 0], [5, 2], [6, 1]];
        let result = Solution::queue_reconstruction(queue);
        println!("RESULT IS {:?} ", result);
    }
}
