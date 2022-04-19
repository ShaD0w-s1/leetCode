/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function (s) {
  let acc = 0;
  const map = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000,
  };
  const SetMap = new Map(Object.entries(map)); // 使用Map替代Object压榨性能
  const len = s.length; // 引用len避免每次都计算
  for (let i = 0; i < len; i++) {
    acc += map[s[i]] < map[s[i + 1]] ? -map[s[i]] : map[s[i]];
  }

  return acc;
};
