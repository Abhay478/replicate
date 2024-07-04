use replicate::replica::*;
use std::{env::args, fs::read_to_string};
fn main() {
    let s = read_to_string(args().nth(1).unwrap()).unwrap();
    dbg!(FileParser::new().parse(&s).unwrap());
}
