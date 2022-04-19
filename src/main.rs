use std::collections::HashMap;

struct Solution {}

impl Solution {
    // 使用 HASH & ITER 方案
    pub fn roman_to_int(s: String) -> i32 {
        let mut acc = 0;
        let mut per = 0;
        let map: HashMap<_, _> = ['I', 'V', 'X', 'L', 'C', 'D', 'M']
            .iter()
            .zip([1, 5, 10, 50, 100, 500, 1000])
            .collect();

        for x in s.chars() {
            println!("acc{}, x{}, per{}", acc, map[&x], per);
            if per == 0 {
                acc = acc + map[&x]
            } else if per >= map[&x] {
                acc = acc + map[&x];
                per = map[&x]
            } else {
                acc = acc - map[&x];
                per = map[&x]
            }
        }
        acc
    }

    // 使用 HASH & 下标 方案

    // 使用 模式匹配 & ITER 方案

    // 使用 模式匹配 & 下标 方案
}

fn main() {
    let i = Solution::roman_to_int(String::from("MCMXCIV"));
    print!("{}", i)
}
