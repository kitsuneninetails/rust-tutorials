fn sum(x: i32) -> impl Fn(i32) -> i32 {
    move |b| x + b
}

fn main() {
    println!("sum_direct: {}", sum(1)(2));
    let add1 = sum(1);
    println!("add1: {}", add1(2));
}
