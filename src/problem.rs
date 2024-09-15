use rayon::join;
/// <summary>
/// Sort the given array in ascending order
/// At least, should beat the default sorting algorithm of the Rust sort
/// </summary>
/// <param name="arr"> array to be sorted in ascending order </param>
/// <param name="size"> array size </param>
///
pub fn giga_sort(arr: &mut Vec<f32>, size: usize) -> &mut Vec<f32> {
    quick_sort(arr as &mut [f32], 0, size - 1);
    return arr;
}

const THRESHOLD: usize = 200;

fn quick_sort(arr: &mut [f32], start: usize, end: usize) -> () {
    if start < end {
        let mut pivot_index: usize = divide(arr, start, end);

        if pivot_index < (arr.len() / 2) as usize {
            pivot_index += 1;
        }

        if (end - start) > THRESHOLD {
            let (first_half, second_half) = arr.split_at_mut(pivot_index);
            join(
                || quick_sort(first_half, 0, first_half.len() - 1),
                || quick_sort(second_half, 0, second_half.len() - 1),
            );
        } else {
            for i in start + 1..=end {
                let mut j = i;
                while j > start && arr[j] < arr[j - 1] {
                    arr.swap(j, j - 1);
                    j -= 1;
                }
            }
        }
    }
}

fn divide(arr: &mut [f32], start: usize, end: usize) -> usize {
    let mid: usize = (start + end) / 2;

    if arr[start] > arr[mid] {
        arr.swap(start, mid);
    }

    if arr[start] > arr[end] {
        arr.swap(start, end);
    }

    if arr[mid] > arr[end] {
        arr.swap(mid, end);
    }

    let pivot: f32 = arr[mid];
    arr[mid] = arr[start];
    arr[start] = pivot;

    let mut leftmark: usize = start + 1;
    let mut rightmark: usize = end;

    while leftmark <= rightmark {
        while leftmark <= rightmark && arr[leftmark] <= pivot {
            leftmark += 1;
        }
        while rightmark >= leftmark && arr[rightmark] >= pivot {
            rightmark -= 1;
        }

        if leftmark <= rightmark {
            (arr[rightmark], arr[leftmark]) = (arr[leftmark], arr[rightmark]);
        }
    }

    arr.swap(start, rightmark);

    rightmark
}
