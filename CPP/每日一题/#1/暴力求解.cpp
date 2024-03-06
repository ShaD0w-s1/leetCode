#include <vector>
using namespace std;
vector<int> twoSum(vector<int> &nums, int target) {
  int i, j;
  int length = nums.size() - 1;
  for (i = 0; i < length; i++) {
    for (j = i + 1; j < length + 1; j++) {
      if (nums[i] + nums[j] == target) {
        return {i, j};
      }
    }
  }
  return {i, j};
};