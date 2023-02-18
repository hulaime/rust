use std::fmt::Display;
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let string3 = String::from("dsafsadf");
    // let result = longest(string1.as_str(), string2);
    let result = longest_with_an_announcement(string1.as_str(), string2, string3);
    println!("The longest string is {}", result);
}
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
