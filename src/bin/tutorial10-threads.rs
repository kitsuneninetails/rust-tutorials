use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;

fn enter_val(chan: Sender<String>, rchan: Receiver<bool>) {
    loop {
        print!("In>");
        stdout().flush().unwrap();
        let mut instr = String::new();
        stdin().read_line(&mut instr).unwrap();
        chan.send(instr.trim().to_string()).unwrap();
        if rchan.recv().unwrap() {
            break;
        }
    }
}

fn print_out_val(chan: Receiver<String>, rchan: Sender<bool>) {
    loop {
        let printstr = chan.recv().unwrap();
        if printstr == "exit!" {
            rchan.send(true).unwrap();
            break;
        }
        println!("Out> {}", printstr.trim());
        stdout().flush().unwrap();
        rchan.send(false).unwrap();
    }
}

fn main() {
    let (s, r) = channel();
    let (s2, r2) = channel();
    let h = spawn(move || {
        print_out_val(r, s2);
    });
    enter_val(s, r2);
    h.join().unwrap();
}
