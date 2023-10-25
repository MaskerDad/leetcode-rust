/*
    题目:判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

示例 1:
输入: 121
输出: true

示例 2:
输入: -121
输出: false
解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。

示例 3:
输入: 10
输出: false
解释: 从右向左读, 为 01 。因此它不是一个回文数。

*/

//#![allow(unused_variables)]
#![allow(unused)]

pub fn is_palindrome(x: i32) -> bool {
    // [0,10)之间的数 都为 true
    if x < 10 && x >=0 {return true;}
    //如果 -开头的 和 100、1000 这样的直接排除
    if x < 0 || x % 10 ==0 { return false }
    let mut tmp = x;
    let mut tmp1 = 0;
    while tmp >= 1 {
        //将数字颠倒  123 先取出 3* 10 在把tmp更新成 12
       //这里加 if 是因为 如果是10位数 最后一次个位数再乘以10 -> 11位数 就超出 i32 的最大值了 就会报错!
        if  tmp < 10 { tmp1 = tmp1 + tmp % 10 }
        else{
            // 123 第一次  temp1 30 tmp 12 -> 第二次 320 1 -> 第三次 3210 0.1 小于1 跳出循环 这样就完成了颠倒数字
            tmp1 = (tmp1 + tmp % 10 )* 10;
        }
        tmp = tmp / 10;
    }
    //123 -> 321 不相等 121 -> 121 相等
    if tmp1 == x {
        return true;
    }
    return false;
}

//测试代码
#[test]
fn pal() {
    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(-121));
    assert_eq!(false, is_palindrome(10));
}
