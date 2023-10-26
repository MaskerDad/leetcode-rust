/*
给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
你可以按任意顺序返回答案。

示例 1：
输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。

示例 2：
输入：nums = [3,2,4], target = 6
输出：[1,2]

示例 3：
输入：nums = [3,3], target = 6
输出：[0,1]

*/
use std::collections::HashMap;

//#![allow(unused_variables)]
#[allow(unused)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut idx = HashMap::new();
    for (j, &x) in nums.iter().enumerate() {
        if let Some(&i) = idx.get(&(target - x)) {
            return vec![i as i32, j as i32];
        }
        idx.insert(x, j);
    }
    unreachable!()
}

#[test]
fn test_func() {
    let nums_1 = vec![2, 7, 11, 15];
    let nums_2 = vec![3, 2, 4];
    let nums_3 = vec![3, 3];
    let res_1 = vec![0, 1];
    let res_2 = vec![1, 2];
    let res_3 = vec![0, 1];
    assert_eq!(res_1, two_sum(nums_1, 9));
    assert_eq!(res_2, two_sum(nums_2, 6));
    assert_eq!(res_3, two_sum(nums_3, 6));
}
