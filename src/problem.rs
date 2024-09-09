/// <summary>
/// Sort the given array in ascending order
/// At least, should beat the default sorting algorithm of the Rust sort
/// </summary>
/// <param name="arr"> array to be sorted in ascending order </param>
/// <param name="size"> array size </param>
pub fn giga_sort(arr: &mut Vec<f32>, size: usize) -> &mut Vec<f32> {
    quick_sort(arr, 0, size - 1, 200);
    return arr;
}
fn quick_sort(arr: &mut Vec<f32>, start: usize, end: usize, threshold: usize) -> () {
    if start < end {
        if (end - start) + 1 <= threshold {
            insertion_sort(arr, start, end);
        } else {
            let pivot_index: usize = divide(arr, start, end);

            quick_sort(arr, start, pivot_index - 1, threshold);
            quick_sort(arr, pivot_index + 1, end, threshold);

            // Parallel.Invoke(
            // () => QuickSort(arr, start, pivotIndex - 1, threshold),
            // () => QuickSort(arr, pivotIndex + 1, end, threshold)
            // );
        }
    }
}

fn divide(arr: &mut Vec<f32>, start: usize, end: usize) -> usize {
    let pivot: f32 = arr[end];
    let mut swap_out: usize = start - 1;

    for left in start..end {
        if arr[left] <= pivot {
            swap_out += 1;
            swap(arr, left, swap_out);
        }
    }

    swap(arr, swap_out + 1, end);
    return swap_out + 1;
}

fn insertion_sort(arr: &mut Vec<f32>, start: usize, end: usize) -> () {
    for i in start + 1..=end {
        let index: f32 = arr[i];
        let mut j: usize = i - 1;

        while j >= start && arr[j] > index {
            arr[j + 1] = arr[j];
            j -= 1;
        }

        arr[j + 1] = index;
    }
}

fn swap(array: &mut Vec<f32>, i: usize, j: usize) -> () {
    let temp: f32 = array[i];
    array[i] = array[j];
    array[j] = temp;
}
