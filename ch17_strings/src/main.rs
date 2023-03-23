fn main() {
    let s = "xyzabc".to_string();
    let st = &s[0..3];
    let et = &s[3..6];

    println!("st: {st}");

    //let x = s;
    // println!("st: {st}");
    //println!("{}", x);

    let ss = et.to_string();
    println!("{}", ss);
}

