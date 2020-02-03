fn divide(divisor: i32, dividend: i32) -> Option<i32> {
    if dividend == 0 {
        None
    } else {
        Some(divisor / dividend)
    }
}

fn print_num(num: Option<i32>) {
    match num {
        Some(n) => println!("{}", n),
        None => println!("NaN")
    };
}

fn flip_sign(num: i32) -> Result<i32, i32> {
    if num == 0 {
        Err(num)
    } else {
        Ok(num * -1)
    }
}

fn flip_test() -> Result<i32, i32> {
    let val1 = match flip_sign(-5) {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };

    println!("Flip_test1: {}", val1);

    // The below 'try' macro is deprecated shorthand for the above construct:
    println!("Flip_test2: {}", r#try!(flip_sign(10)));

    // The below "?" operator is current shorthand for the above construct:
    println!("Flip_test2: {}", flip_sign(10)?);

    // This will trigger the "return Err" condition and return the error without even printing out.
    println!("Flip_test3 (WON'T PRINT): {}", flip_sign(0)?);
    println!("This won't run!");
    flip_sign(23)
}

fn main() {
    print_num(divide(35, 5));
    print_num(divide(35, 0));

    let r = flip_sign(0);
    match r {
        Ok(n) => print_num(Some(n)),
        Err(e) => println!("Error: {}", e)
    };

    print_num(flip_sign(-32).ok());
    print_num(Some(flip_sign(0).unwrap_err()));
    print_num(Some(flip_sign(0).unwrap_or(-65535i32)));

    flip_test();

    println!("{}", flip_sign(0).unwrap()); // This will panic.
    println!("{}", flip_sign(35).unwrap_err()); // This will also panic.
}
