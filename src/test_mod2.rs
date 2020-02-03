use chrono::Utc;

pub mod test_funcs {
    fn hello_string(name: String) -> String {
        format!("Hello, {}, the time is {}", name, super::Utc::now().to_rfc3339())
    }

    pub fn print_hello() {
        println!("{}", hello_string("Mr. Engineer".to_string()));
    }
}
