use std::{thread, fmt::Error, panic};

fn main() {
    let _t = 23;
    let _c = 0;

    let h = 
        thread::spawn(move || pirate_share(t, c));
    
    match h.join() {
        Ok(v) => println!("v: {}", v),
        Err(_) => println!("Error"),
    }
    
    let res = panic::catch_unwind(|| println!("U: {}", unrap_3()));
    match res {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Err"),
    }   

    print!("Ajunge...");
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let h = total / 2;
    h / crew_size as u64    
}

fn unrap_3() -> u32 {
    unrap_2()
}  

fn unrap_2() -> u32 {
    unrap_1("dff10").unwrap()
}

fn  unrap_1(x: &str) -> Result<u32, std::num::ParseIntError> {
    let z = x.parse::<u32>()?;
    Ok(z)
}