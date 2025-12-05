// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
// You must write an algorithm that runs in O(n) time and without using the division operation.
//
// Example 1:
// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:
// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
fn arr_product_except_itself(nums: Vec<i32>) {
    let N: usize = nums.len();
    let mut left: Vec<i32> = vec![0; N];
    left[0] = 1;
    let mut l_temp: i32 = *nums.get(0).unwrap();
    for i in 1..N {
        left[i] = l_temp;
        l_temp *= nums.get(i).unwrap();
    }

    let mut right: Vec<i32> = vec![0; N];
    right[N-1] = 1;
    let mut r_temp: i32 = *nums.last().unwrap();
    for i in (0..N-1).rev() {
        right[i] = r_temp;
        r_temp *= *nums.get(i).unwrap();
    }

    let mut res: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        res.insert(i, left.get(i).unwrap() * right.get(i).unwrap());
    }

    println!("left = {:?}", left);
    println!("right = {:?}", right);
    println!("res = {:?}", res);
}

#[cfg(test)]
mod tests {
    use crate::array::arr_product_except_itself::arr_product_except_itself;

    #[test]
    fn test_arr_product_except_itself() {
        arr_product_except_itself(vec![1,2,3,4].to_vec());
    }
}