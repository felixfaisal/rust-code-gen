use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

enum Felix {
    ICMP = 1,
    IGP = 3,
}
impl Felix {
    pub fn String(&self) {
        match self {
            Felix::ICMP => println!("ICMP"),
            Felix::IGP => println!("IGP"),
            _ => println!("Protocol does not exist"),
        }
    }
    pub fn value(&self) {
        match self {
            Felix::ICMP => println!("{}", Felix::ICMP as u32),
            Felix::IGP => println!("{}", Felix::IGP as u32),
            _ => println!("Protocol does not exist"),
        }
    }
    pub fn new(u: u32) -> Felix {
        match u {
            1 => Felix::ICMP,
            3 => Felix::IGP,
            _ => Felix::IGP,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct union {
    XXX_unionData: [u8; 4],
}
impl union {
    pub fn new() -> union {
        union {
            XXX_unionData: [192, 168, 225, 0],
        }
    }
    pub fn write(&mut self) {
        for (pos, _e) in self.XXX_unionData.iter().enumerate() {
            self.XXX_unionData[pos] = 0;
        }
    }
}

fn main() {
    println!("Generating Rust code ===========>");
    // generate_enum();
    // run_shell();
    let mut uniondata = union::new();
    dbg!(uniondata.XXX_unionData);
    uniondata.write();
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
