use crate::enum_test::Felix;
use crate::header::*;
use crate::union_test::union;
use std::io::prelude::*;
use std::process::Command;
mod enum_test;
mod header;
mod shell;
mod types;
mod union_test;

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
