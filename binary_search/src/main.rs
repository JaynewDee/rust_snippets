fn main() {
    let test_nums = [3, 16, 25, 33, 42, 55, 78, 93, 94, 96, 99];
    let nums_length: usize = test_nums.len() as usize;
    let target: i32 = 42;

    let is_found: Option<usize> = binary_search(&test_nums, nums_length, &target);
    println!("{:?}", is_found)
}

pub fn binary_search(a: &[i32], len: usize, target_value: &i32) -> Option<usize> {
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = &a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if val > target_value {
            high = mid - 1;
        }
    }
    None
}
