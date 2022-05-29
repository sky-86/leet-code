// O(2)
fn _missing_number(nums: Vec<i32>) -> i32 {
    let actual_sum: i32 = nums.iter().sum();
    let expected_sum: i32 = (1..=nums.len() as i32).sum();
    expected_sum - actual_sum
}

// O(1)
fn missing_number(nums: Vec<i32>) -> i32 {
    let expected_sum: i32 = ((nums.len() as f32 / 2.0) * (nums.len() as f32 + 1.0)) as i32;
    let actual_sum: i32 = nums.iter().sum();
    expected_sum - actual_sum
}

fn main() {
    let nums = vec![0, 1];
    let n = missing_number(nums);
    println!("{}", n);
}
