fn quicksort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);

    quicksort(&mut arr[..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_test() {
        let mut vec = vec![1, 5, 2, 7, 4, 4, 7, 4, 3];

        println!("{:?}", vec);

        quicksort(vec.as_mut_slice());

        for v in vec {
            print!("{} ", v);
        }

        println!();
    }
}
