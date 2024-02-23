mod linear_search_list;

fn main() {
    let haystack = [1, 2, 3, 4, 5];
    let needle = 2;

    println!(
        "{}",
        linear_search_list::linear_search_list(haystack, needle)
    );
}
