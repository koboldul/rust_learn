use std::collections::HashMap;
use webx::{BasicRouter, Response};

mod webx;

fn main() {
    let mut router = BasicRouter::new();

    router.add_route("/", |_| { 
        Response {
            code: 200,
            headers: HashMap::new(),
            body: Vec::new()
        }
    });
}

fn other_stuff() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);

    clean(f);

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("i: {}", i);
    };

    call_twice(incr);

    let mut greeting = String::from("Hello, ");

    let mut cc = greeting.clone();
    let mut greet = move || {
        cc.push_str("world!");
        println!("{}", cc);
    };

    greet();
}

fn clean<A>(action: A) where A: FnOnce() {
    action();
}

fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}
