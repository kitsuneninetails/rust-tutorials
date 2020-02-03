use chrono::Utc;

fn hello_string(name: String) -> String {
    format!("Hello, {}, the time is {}", name, Utc::now().to_rfc3339())
}

pub fn print_hello() {
    println!("{}", hello_string("Mr. Developer".to_string()));
}

