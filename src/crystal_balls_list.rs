use num_integer;

// You have two glass balls that break from a certain height.
// You are in a building with N floors and you have to calculate
// the first building from which the balls will break.

pub fn crystal_balls_list(arr: &[bool]) -> i32 {
    let elements_count = arr.len();
    let offset = num_integer::sqrt(elements_count);
    let mut current_floor = 0;

    while current_floor < elements_count {
        if is_crystal_broken(current_floor, arr) {
            break;
        } else {
            current_floor = current_floor + offset;
        }
    }

    found_first_broken_floor(current_floor - offset, arr)
}

fn found_first_broken_floor(mut floor: usize, arr: &[bool]) -> i32 {
    while !is_crystal_broken(floor, arr) {
        if floor == arr.len() - 1 {
            return -1;
        } else {
            floor = floor + 1;
        }
    }

    floor as i32
}

fn is_crystal_broken(floor: usize, arr: &[bool]) -> bool {
    arr[floor]
}
