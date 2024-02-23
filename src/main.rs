mod binary_search_list;
mod crystal_balls_list;
mod linear_search_list;

fn main() {
    let mut haystack: [bool; 1000000] = [false; 1000000]; // Inicializa todos los elementos a false
    haystack[999998] = true;

    println!(
        "crystal ball breaks at floor: {}",
        crystal_balls_list::crystal_balls_list(&haystack)
    );
}
