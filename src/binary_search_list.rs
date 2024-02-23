pub fn binary_search_list<T: Ord>(arr: &[T], target: &T) -> bool {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        let mid_val = &arr[mid];

        if mid_val == target {
            return true;
        } else if mid_val < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    false
}
