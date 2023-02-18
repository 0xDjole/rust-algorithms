use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    index: usize,
    distance: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solution(adj_list: &Vec<Vec<(usize, i32)>>, start: usize) -> HashMap<usize, i32> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        index: start,
        distance: 0,
    });

    while let Some(Node { index, distance }) = heap.pop() {
        if distances.contains_key(&index) {
            continue;
        }
        distances.insert(index, distance);

        for &(next_index, weight) in &adj_list[index] {
            heap.push(Node {
                index: next_index,
                distance: distance + weight,
            });
        }
    }

    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra() {
        let adj_list = vec![
            vec![(1, 3), (2, 9)],
            vec![(3, 7)],
            vec![(3, 2)],
            vec![(4, 3)],
            vec![],
        ];

        let distances = solution(&adj_list, 0);

        println!("Distances from node 0:");
        for (node, distance) in distances {
            println!("{}: {}", node, distance);
        }
    }
}
