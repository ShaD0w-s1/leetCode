fn main() {
    // 反转对比法
    impl Solution {
        pub fn is_palindrome(x: i32) -> bool {
            // 处理特殊情况
            if x < 0 || x % 10 == 0 && x != 0 {
                return false;
            }
            let mut y = x;
            // 创建一个变量留存反转后的数（对10取余并保存）
            let mut rev = 0;
            // 往复直到 原数比反转后小 (如果小了说明已经翻转了一半或者多)
            while y > rev {
                rev = rev * 10 + y % 10; // 翻转算法
                y /= 10;
            }
            return y == rev || y == rev / 10; // 判断是否等于 这时有可能因为是长度位奇数导致不等于 除10去减少一位再尝试
        }
    }
}
