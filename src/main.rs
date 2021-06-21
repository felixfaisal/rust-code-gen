use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    enum Felix {
        ICMP = 1,
        IGP = 3,
    };
    println!("Generating Rust code ===========>");
    // generate_enum();
    // run_shell();
    let proto = Felix::ICMP;
    let mut curproto = 1;
    let newproto = match curproto {
        1 => Felix::ICMP,
        3 => Felix::IGP,
        _ => Felix::ICMP,
    };
    dbg!(newproto as i32);
}

fn generate_enum() {
    let mut file = File::create("binapi/generated.rs").unwrap();
    file.write_all(b"fn generated() {   \n").unwrap();
    file.write_all(b"   enum felix {    \n").unwrap();
    file.write_all(b"       faisal      \n").unwrap();
    file.write_all(b"       faisax      \n").unwrap();
    file.write_all(b"   };              \n").unwrap();
    file.write_all(b"}                  \n").unwrap();
}
fn run_shell() {
    let mut echo_hello = Command::new("firefox");
    //echo_hello.arg("-c").arg("echo hello");
    let hello_1 = echo_hello.output().expect("failed to execute process");
    // let hello_2 = echo_hello.output().expect("failed to execute process");
    dbg!(hello_1.stdout);
    // dbg!(hello_2.stdout);
}
