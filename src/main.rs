use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    println!("Generating Rust code ===========>");
    generate_enum();
    run_shell();
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
