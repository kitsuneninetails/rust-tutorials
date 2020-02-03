#[derive(Debug)]
pub enum DivErr {
    Overflow,
    DivideBy0,
}

fn divide(divisor: i32, dividend: i32) -> Result<f64, DivErr> {
    if dividend == 0 {
        Err(DivErr::DivideBy0)
    } else {
        Ok(divisor as f64/ dividend as f64)
    }
}

fn divide_safe(divisor: i32, dividend: i32) -> Result<f64, f64> {
    if dividend == 0 {
        Err(0f64)
    } else {
        Ok(divisor as f64/ dividend as f64)
    }
}

fn factors(num: i32) -> Option<Vec<i32>> {
    if num == 0 {
        None
    } else {
        let mut ret = vec![];
        for i in 1..((if num > 0 { num } else { num * -1 })/2) + 1 as i32 {
            if num % i == 0 {
                ret.push(i);
            }
        }
        ret.push(num);
        Some(ret)
    }
}

fn sum(v: Vec<i32>) -> i32 {
    let mut n = 0;
    for i in v {
        n += i
    }
    n
}

fn flip_sign(num: i32) -> Result<i32, i32> {
    if num == 0 {
        Err(num)
    } else {
        Ok(num * -1)
    }
}

fn main() {
    println!("== Options =================================");

    let f = factors(10); // creates a Some(vector)
    let s = f.map(|n| sum(n)); // Maps the vector in the Option to int
    println!("Sum of valid factors = {:?}", s);

    let f = factors(0); // creates a None
    let s = f.map(|n| {
        println!("I won't run this!");
        sum(n)
    }); // does nothing because the Option is already None
    println!("Sum of invalid factors = {:?}", s);

    let f = factors(0); // creates a None
    let s = f.map_or(0, |n| {
        println!("I won't run this!");
        sum(n)
    }); // returns default of "0" because option is None
    println!("Sum of invalid factors (with default) = {:?}", s);

    let f = factors(10); // creates a Some(vector)
    let s = f.and_then(|n| factors(sum(n))); // Runs a function using the vector which returns Some(f64)
    println!("Sum of valid factors of sum of factors = {:?}", s);

    let f = factors(0); // creates a None
    let s = f.and_then(|n| {
        println!("I won't run this!");
        factors(sum(n))
    }); // does nothing because the Option is already None
    println!("Sum of invalid factors of sum of factors / 100 = {:?}", s);

    println!("== Results =================================");

    let f = divide(100, 2); // creates an Ok(f64)
    let s = f.map(|d| factors(d.floor() as i32)); // maps the f64 to a vector of ints
    println!("Factors of good division = {:?}", s);

    let f = divide(100, 0); // creates a Err(DivErr)
    let s = f.map(|d| {
        println!("I won't run this!");
        factors(d.floor() as i32)
    }); // Does nothing because result was error
    println!("Factors of invalid division = {:?}", s);

    let f = divide(100, 2); // creates an Ok(f64)
    let s = f.and_then(|d| divide(d.floor() as i32, 2)); // runs another divide with the previous result
    println!("Factors of good division and division = {:?}", s);

    let f = divide(100, 0); // creates a Err(DivErr)
    let s = f.and_then(|d| {
        println!("I won't run this!");
        divide(d.floor() as i32, 2)
    }); // Does nothing because result was error
    println!("Factors of invalid division and division = {:?}", s);

    let f = divide(100, 0); // creates a Err(DivErr)
    let s = f.or_else(|d| divide_safe(100,1)); // Recovers by returning an Ok(f64)
    println!("Factors of invalid division good recovery = {:?}", s);

    let f = divide(100, 0); // creates a Err(DivErr)
    let s = f.or_else(|d| divide_safe(100,0)); // Fails recovery, but error type is now Err(f64)
    println!("Factors of invalid division recovery error = {:?}", s);

    println!("== Vectors =================================");
    let vec = vec![0, 2, 0, 4, 0, 6, 0, 8, 0, 10];
    let vec2: Vec<Result<f64, DivErr>> = vec.into_iter()
        .map(|i| divide(10, i))
        .collect();
    println!("Vec with map: {:?}", vec2);

    let vec = vec![0, 2, 0, 4, 0, 6, 0, 8, 0, 10];
    let mut vec2: Vec<i32> = vec.into_iter()
        .map(|i| divide(10, i))
        .filter(|j| j.is_ok())
        .map(|div| factors(div.unwrap().floor() as i32))
        .filter(|j| j.is_some())
        .map(|fact| sum(fact.unwrap()))
        .collect();
    println!("Vec with multiple maps and filter: {:?}", vec2);

    let vec = vec![0, 2, 0, 4, 0, 6, 0, 8, 0, 10];
    let vec2: Vec<i32> = vec.into_iter()
        .flat_map(|i| divide(10, i))
        .flat_map(|div| factors(div.floor() as i32))
        .map(|f| sum(f))
        .collect();
    println!("Vec with flatmap/map combo: {:?}", vec2);

}
