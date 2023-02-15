

fn main() {
    let my_string = String::from("hello world");
    let word = first_word_1(&my_string);
    let word = first_word_2(&my_string);
    // `first_word` 接受 `String` 的切片，无论是部分还是全部

    // let word = first_word_1(&my_string[0..6]); // expected struct `String`, found `str`
    // let word = first_word_1(&my_string[..]); // expected struct `String`, found `str`
    let word = first_word_2(&my_string[0..6]);
    let word = first_word_2(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    

    let my_string_literal = "\ta\tbhello world";

    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    // let word = first_word_1(&my_string_literal[0..6]); // expected struct `String`, found `str`
    // let word = first_word_1(&my_string_literal[..]); // expected struct `String`, found `str`
    // let word = first_word_1(my_string_literal); // expected struct `String`, found `str`

    let word = first_word_2(&my_string_literal[0..6]);
    let word = first_word_2(&my_string_literal[..]);
    // 因为字符串字面值**就是**字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word_2(my_string_literal);

    // 总结
    // &String 允许 &String，&str 允许 &String和&str
    let word = first_word_3(my_string_literal);
    println!("{}", word);
}


fn first_word_1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}", item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_3(s: &str) -> &str {
    // let bytes = s.as_bytes();

    // for item in s.split_whitespace() {
    //     println!("{}", item);
    //     if item == ' ' {
    //         return &s[0..8];
    //     }
    // }

    // &s[..]
    s.split_whitespace().next().unwrap_or(&s[..])
}

