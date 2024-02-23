pub fn linear_search_list(haystack: [i32; 5], needle: i32) -> bool {
    let mut result = false;
    for hay in haystack.into_iter() {
        if hay == needle {
            result = true;

            return result;
        }
    }

    result
}
