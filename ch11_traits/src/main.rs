use std::fs::File;
use std::io::Write;

use vegetable::{Tomato, QCumber, Salad, GSalad, Vegetable};

mod vegetable;

fn main() {
    // writer_trait();
    make_salad();
}

fn make_salad() {
    let tomato = Tomato { color: "red".to_string() };
    let qc = QCumber { color: "green".to_string() };
    let tomato2 = grow_veg();

    let salad = Salad{ vegetables: vec![tomato2] };
    let gs = GSalad{ vegetables: vec![Box::new(tomato), Box::new(qc)] };
    salad.make_it();
    gs.make_it();
}

fn grow_veg() -> impl Vegetable {
    Tomato { color: "red".to_string() }
}

fn writer_trait() {
    let mut lf = File::create("hello.txt").unwrap();
    let w: Box<dyn Write> = Box::new(lf);
    // say_hello()).unwrap();
}

fn say_hello(writer: &mut dyn Write) -> std::io::Result<()> {
    writer.write_all(b"Hello, world!")?;
    writer.flush()
}

fn say_hello_w<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello, world!")?;
    out.flush()
}