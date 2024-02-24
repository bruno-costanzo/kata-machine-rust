mod binary_search_list;
mod bubble_sort_list;
mod crystal_balls_list;
mod linear_search_list;

fn main() {
    bubble_sort_list();
}

fn bubble_sort_list() {
    let arr = [1, 3, 7, 4, 2];

    println!("sorted: {:?}", bubble_sort_list::bubble_sort_list(&arr));
}
