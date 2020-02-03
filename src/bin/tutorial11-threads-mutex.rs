use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn enter_val(data: Arc<Mutex<Option<String>>>) {
    loop {
        print!("In>");
        stdout().flush().unwrap();
        let mut instr = String::new();
        stdin().read_line(&mut instr).unwrap();

        let s = instr.trim().to_string();
        data.lock().unwrap().replace(s.clone());
        if s == "exit!" {
            break;
        }
    }
}

fn print_out_val(data: Arc<Mutex<Option<String>>>) {
    loop {
        let printstr = data.lock().unwrap().take();
        if let Some(s) = printstr {
            if s == "exit!" {
                break;
            }

            println!("Out> {}", s.trim());
            stdout().flush().unwrap();
        }
        sleep(Duration::from_millis(2000));
    }
}

fn main() {
    let data = Arc::new(Mutex::new(None));
    let data2 = data.clone();
    let h = spawn(move || {
        print_out_val(data2);
    });

    // This will fail to compile due to using moving `data` into the closure, so `enter_val`
    // cannot use it.
    // let h = spawn(move || {
    //     print_out_val(data);
    // });

    // This will also fail to compile, even if you clone the data, because it still gets
    // moved into the closure.
    // let h = spawn(move || {
    //     print_out_val(data.clone());
    // });

    enter_val(data);
    h.join().unwrap();
}
