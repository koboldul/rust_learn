use std::fmt::Display;

struct RcBox<T: ?Sized> {
    value: T,
}

fn main() {
    let x: RcBox<String> = RcBox {
        value: "lunch".to_string()
    };

    let y: &RcBox<dyn Display> = &x;

    display_it(y)
}

fn display_it(x: &RcBox<dyn Display>) {
    println!("Printed {}", &x.value);
}
