use crate::header::*;
use std::io::prelude::*;
use std::process::Command;
mod header;
mod shell;
mod types;

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
    XXX_unionData: [u8; 16],
}
impl union {
    pub fn new() -> union {
        union {
            XXX_unionData: [0; 16],
        }
    }
    pub fn write(&mut self) {
        for (pos, e) in self.XXX_unionData.iter().enumerate() {
            println!("{} {}", pos, e);
        }
    }
    pub fn update(&mut self) {
        for i in 0..self.XXX_unionData.len() {
            self.XXX_unionData[i] = 70;
        }
    }
    pub fn SetIP4(&mut self) {
        println!("Setting IP4");
        for i in 0..4 {
            match i {
                0 => self.XXX_unionData[i] = 192,
                1 => self.XXX_unionData[i] = 226,
                2 => self.XXX_unionData[i] = 120,
                3 => self.XXX_unionData[i] = 20,
                _ => println!("Reached end of the array"),
            }
        }
    }
    pub fn GetIP4(&self) {
        println!("Fetching IP4");
        for i in 0..4 {
            println!("{}", self.XXX_unionData[i]);
        }
    }
    pub fn SetIP6(&mut self) {
        println!("Setting IP6");
        for i in 0..self.XXX_unionData.len() {
            self.XXX_unionData[i] = 10;
        }
    }
    pub fn GetIP6(&self) {
        println!("Fetching IP6");
        for i in 0..self.XXX_unionData.len() {
            println!("{}", self.XXX_unionData[i]);
        }
    }
}

fn main() {
    println!("Generating Rust code ===========>");
    generate_header();
    let mut uniondata = union::new();
    dbg!(uniondata.XXX_unionData);
    // uniondata.write();
    // uniondata.XXX_unionData[1] = 0;
    //uniondata.write();
    // uniondata.update();
    // uniondata.write();
    uniondata.SetIP4();
    uniondata.GetIP4();
    uniondata.SetIP6();
    uniondata.GetIP6();
}
