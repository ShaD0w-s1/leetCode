fn main() {
    // 双循环
    impl SolutionSum {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut i = 0;
            let mut j = 0;
            let mut ret = Vec::new();
            for i in 0..nums.len() {
                for j in 0..nums.len() {
                    if (i != j) && (j > i) && (nums[i] + nums[j] == target) {
                        ret.push(i as i32);
                        ret.push(j as i32);
                        break;
                    }
                }
            }
            ret
        }
    }

    // 哈希表（不会）
    impl SolutionHash {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {}
    }
}
