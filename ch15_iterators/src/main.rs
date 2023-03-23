use std::iter::from_fn;

use rand::random;

fn main() {
    println!("Hello, world!");

    for i in make_rands(10) {
        println!("HHH: {}", i);
    }
    for f in fibonacci().take(10) {
        println!("FFF: {}", f);
    }

    filter_map()
}

fn make_rands(count: usize) -> impl Iterator<Item=f64>
{
     from_fn(|| Some(random::<f64>()))
    .take(count)
}

fn fibonacci() -> impl Iterator<Item=usize> {
    let mut state = (0, 1);
    from_fn(move || {
        state = (state.1, state.0 + state.1);
        Some(state.0)
    })
}

fn drain_ex() {
    let mut order = "Earth".to_string();
    let inner = String::from_iter(order.drain(1..4));

    assert_eq!(order, "Eh");
    assert_eq!(inner, "art");
}

fn filter_map() {
    let animals = "ponies \n cats \niguanas";

    let v:Vec<&str> = animals.lines()
        .map(str::trim)
        .filter(|s| *s != "cats")
        .collect();

    for a in v{
        print!("animal: {} ", a)
    }
}