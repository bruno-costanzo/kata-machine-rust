pub fn bubble_sort_list(arr: &[i32]) -> Vec<i32> {
    let mut sorted_array = arr.to_vec();
    let mut n = sorted_array.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if sorted_array[i - 1] > sorted_array[i] {
                sorted_array.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }

    sorted_array
}
