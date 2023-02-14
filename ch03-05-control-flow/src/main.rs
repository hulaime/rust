// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { 6.2 };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 66666666666666666666666 };

//     println!("The value of number is: {number}");
// }

// 在运行时确定数字的类型，Rust将无法做到这一点；
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6.0 as i32 };

    println!("The value of number is: {number}");
}

