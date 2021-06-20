use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut file = File::create("binapi/generated.rs").unwrap();
    file.write_all(b"fn generated() {   \n").unwrap();
    file.write_all(b"   enum felix {    \n").unwrap();
    file.write_all(b"       faisal      \n").unwrap();
    file.write_all(b"       faisax      \n").unwrap();
    file.write_all(b"   };              \n").unwrap();
    file.write_all(b"}                  \n").unwrap();
}
