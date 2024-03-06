/*
 * @Author: zhangyuxuan
 * @Date: 2022-09-03 21:49:36
 * @LastEditTime: 2022-09-03 21:57:06
 * @LastEditors: zhangyuxuan
 * @FilePath: \RUST\#3\滑动窗口\main.rs
 */
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let map = std::collections::HashMap::new();
    let mut left = 0;
    let mut max = 0;
    s.iter().enumerate().for_each(|(i, c)| {
      if let Some(&index) = map.get(c) {
        left = std::cmp::max(left, index + 1);
      }
      map.insert(c, i);
      max = std::cmp::max(max, i - left + 1);
    });

  }
}