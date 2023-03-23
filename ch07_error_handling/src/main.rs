use std::thread;

fn main() {
    let t = 23;
    let c = 0;

    let h = 
        thread::spawn(move || pirate_share(t, c));
    
    match h.join() {
        Ok(v) => println!("v: {}", v),
        Err(_) => println!("Error"),
    }

    print!("Ajunge...");
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let h = total / 2;
    h / crew_size as u64    
}