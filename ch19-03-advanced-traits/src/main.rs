trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    // Pilot::fly(&person);
    // Wizard::fly(&person);
    person.fly();
    
    let x = 4;

    let equal_to_x = returns_closure_1();
    let equal_to_y = returns_closure_2();

    let y = 4;

    println!("{}", equal_to_x(x));//5
    println!("{}", equal_to_y(y));//5
}
fn returns_closure_1() -> impl Fn(i32) -> i32 {
    |x| x + 1
    // |z| z == x
}

fn returns_closure_2() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
