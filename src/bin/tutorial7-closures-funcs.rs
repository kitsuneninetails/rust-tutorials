fn op(num1: i32, num2: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(num1, num2)
}

fn op2(num1: i32, num2: i32, op: impl FnOnce(i32, i32) -> i32) -> i32 {
    op(num1, num2)
}

fn main() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    println!("using func pointer: {}", op(2, 3, add));
    println!("using closure: {}", op(20, 30, |a, b| a + b));
    println!("using closure (with trait obj): {}", op2(200, 300, |a, b| a + b));
}
