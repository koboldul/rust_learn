use std::vec;

fn main() {
    let strings: Vec<String> = vec!["1".to_string(), "2".to_string()];
    for s in &strings {
        println!("Stuff {} at address: {:p}", *s, s);
    }                                   
    println!("{} values (s)", strings.len()); 
}
