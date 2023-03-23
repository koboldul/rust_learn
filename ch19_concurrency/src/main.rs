use std::sync::Mutex;

fn main() {
    let mut list = ListP(Mutex::new(vec![1, 2, 3, 4, 5]));

    let mut list2 = list.0.lock().unwrap();
    list2.push(6);
    println!("{:?}", list2);
}

type WaitingList = Vec<u32>;

struct ListP (Mutex<WaitingList>);
