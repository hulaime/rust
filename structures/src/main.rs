fn main() {
    let v = [1, 2, 3, 4, 5];
    let first_even = v.iter().find(|&x| x % 2 == 0);
    
    assert_eq!(first_even, Some(&2));
}
// fn main() {
//     let v = [0, 1, 2, 3, 4, 5, 6, 7];
//     // 错误示例 1
//     // let list1: Vec<_> = v.iter().filter(|i| i % 3 == 0).collect();
//     // let list2 = [0, 3, 6];
//     // assert_eq!(list1, list2);
//     // 错误示例 2
//     // let list1: Vec<_> = v.iter().filter(|i| **i % 3 == 0).collect();
//     // let list2 = [0, 3, 6];
//     // assert_eq!(list1, list2);
//     // 错误示例 3
//     // let list1: Vec<_> = v.iter().filter(|i| **i % 3 == 0).copied().collect();
//     // let list2 = [0, 3, 6];
//     // assert_eq!(list1, list2);
//     // 错误示例 4
//     // let list1: Vec<_> = v.iter().filter(|i| **i % 3 == 0).copied().collect();
//     // assert_eq!(list1, [0, 3, 6]);
//     // 正确示例
//     let list1: Vec<_> = v.iter().filter(|i| **i % 3 == 0).copied().collect();
//     let list2 = vec![0, 3, 6];
//     assert_eq!(list1, list2);
// }
// fn main() {
//     let vec = vec![1, 2, 3, 4, 5];
//     let vec_str: Vec<_> = vec.iter().map(|x| x + 1).collect();
//     assert_eq!(vec_str, [2, 3, 4, 5, 6]);
// }
// // fold函数第一个是初始值，第二个是一个闭包，闭包第一个参数是一个累计值，第二个参数是本次迭代元素的引用，返回值作为下一次迭代的累计值。
// fn main() {
//     let v = [1, 2, 3, 4, 5];

//     let sum: i32 = v.iter().fold(2, |acc, x| acc + x); // 2 + （(((1 + 2) + 3) + 4) + 5）
//     let mul: i32 = v.iter().fold(3, |acc, x| acc * x); // 3 * （(((1 * 2) * 3) * 4) * 5）

//     assert_eq!(sum, 17);
//     assert_eq!(mul, 360);
// }
