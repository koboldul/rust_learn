mod queue;
use std::rc::Rc;
use queue::Queue;
mod spider_robot;

use spider_robot::SpiderRobot;

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BromIntent
}

#[derive(Copy, Clone)]
enum BromIntent { Fetch, Dump }

fn main() {
    //test_q();
    let spider = SpiderRobot::new()
    // test_broom();
}

fn test_broom() {
    println!("Hello, world!");
    let b = Broom {
        name: "Hokey".to_string(),
        health: 32,
        height: 32,
        intent: BromIntent::Dump,
        position: (1.0, 1.0, 1.0)
    };

    let (b1,b2) = chop(b);
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut b1 = Broom { height: b.height / 2 , ..b};
    let mut b2 = Broom { name: b1.name.clone(), ..b1};

    b1.name.push_str("I");
    b2.name.push_str("II");

    (b1, b2)
}

fn test_q() {
    let mut q = Queue::new();

    q.push(23);

    let mut pq = Box::new(Queue::new());
    pq.push('■');

    let rq = Rc::new(Queue::<i32>::new());
    // Rc is not mutable
    // rq.push("jijij".to_string());
    println!("is empty {}", rq.is_empty());
}

#[test]
fn test_q_test() {
    test_q()
}
