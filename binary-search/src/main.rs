fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for target in [1, 7, 10, 0, 11] {
        match binary_search(&nums, target) {
            Some(i) => println!("{target:>2} -> index {i}"),
            None => println!("{target:>2} -> not found"),
        }
    }
}

fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}
