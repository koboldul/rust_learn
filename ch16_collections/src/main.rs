fn main() {
    vec_last();
}

fn vec_last() {
    let mut slice = [1,2,3,4,5];
    {
        let last = slice.last_mut().unwrap();
        
        *last = 42;

        let t = slice[0];
        println!("t: {}", t);
    }
}
