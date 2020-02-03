fn takes_value(foo: String) {
    println!("{}", foo);
}

fn takes_returns_value(foo: String) -> String {
    println!("Happy decorations!!! {}", foo);
    foo
}

fn takes_ref(foo: &String) {
    println!("Happy decorations!!! {}", foo);
}

fn takes_ref_returns(foo: &String) -> String {
    format!("{}-foo", foo)
}

fn takes_mut(mut foo: String) -> String {
    foo.remove(0);
    foo
}

fn takes_mut_ref(foo: &mut String) {
    foo.remove(0);
}

// This func won't compile.  What is the problem?
// fn returns_ref() -> &String {
//     let str1 = "foo".to_string();
//     &str1
// }

// This func won't compile.  What is the problem?
// fn returns_ref<'a>() -> &'a String {
//     let str1 = "foo".to_string();
//     &str1
// }

fn returns_ref(input: &String) -> &String {
    input
}

fn returns_mut_ref(input: &mut String) -> &mut String {
    input
}

fn main() {
    let _str1 = "foo".to_string();
    //_str1 = "bar".to_string(); // Doesn't work because str1 wasn't declared mutable
    //_str1.remove(0); // Doesn't work because str1 wasn't declared mutable

    let mut str1: String = "foo".to_string();
    println!("{}", str1);
    str1 = "barbarbar".to_string();
    println!("{}", str1);

    str1.remove(0);
    println!("{}", str1);
    takes_value(str1);

    println!("{}", str1); // Doesn't work because str1 has been moved

    let str1 = "foo".to_string();
    let str2 = takes_returns_value(str1);

    // println!("{}", str1); // Doesn't work because str1 has been moved
    println!("{}", str2);

    let str1 = "foo".to_string();
    let str1 = takes_returns_value(str1);
    println!("{}", str1); // Works!  Why?

    let str1 = "foo".to_string();
    takes_ref(&str1);
    println!("{}", str1);

    let str1 = "foo".to_string();
    let str2 = takes_ref_returns(&str1);
    println!("{}", str1); // Works; we can still use str1, why?
    println!("{}", str2);

    let str1 = "foo".to_string();
    let str2 = takes_mut(str1); // Works even though str1 wasn't declared mut!  Why?
    // println!("{}", str1); // Doesn't work because str1 has been moved
    println!("{}", str2);

    let _str1 = "foo".to_string();
    //takes_mut_ref(&_str1); // Doesn't work because str1 needs to be mut ref
    //takes_mut_ref(&mut _str1); // Doesn't work because str1 wasn't declared mutable

    let mut str1 = "bar".to_string();
    takes_mut_ref(&mut str1);
    println!("{}", str1);

    let mut str1 = "bamf".to_string();
    {
        // Can have any number of reading references, but only one mutating reference.
        let str3 = &str1;
        let mut str2 = returns_ref(&str1);
        //takes_mut_ref(&mut str2); // Doesn't work because we cannot borrow mutably from an existing reference

        str1 = "bar".to_string(); // Doesn't work because str2 is "borrowing" str1's reference
        println!("{}", str1);
        //println!("{}", str2);

        str1 = "bar".to_string(); // Works because str2 isn't used anymore and has released the reference
        println!("{}", str1);

    }
    str1 = "bar".to_string();
    println!("{}", str1);

    let mut str1 = "foo".to_string();
    {
        let mut str2: &mut String = returns_mut_ref(&mut str1);
        // Can have any number of reading references, but only one mutating reference!
        takes_mut_ref(&mut str2); // Works!

        //println!("{}", str1); // Doesn't work because str2 is borrowing str1's reference as mutable
        println!("{}", str2);
        //str1 = "bar".to_string(); // Doesn't work because str2 is borrowing str1's reference
        *str2 = "bar".to_string(); // Works because str2 is borrowing as mutable, so it can change the value
        println!("{}", str1); // Works because str2 isn't used anymore and can release the reference
    }
}
