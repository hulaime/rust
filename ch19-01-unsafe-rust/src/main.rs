#![allow(unused)]
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    num += 1;
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    unsafe {
        // *r1+=1;
        println!("r1 is: {}", *r1);
        *r2 += 1;
        println!("r2 is: {}", *r2);
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
