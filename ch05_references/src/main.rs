use std::borrow::{Cow, self};

/// https://doc.rust-lang.org/std/borrow/enum.Cow.html#impl-Borrow%3CB%3E-for-Cow%3C%27a%2C%20B%3E

fn main() {
    test_cow();
}

struct Items<'a, X: 'a> where [X]: ToOwned<Owned = Vec<X>> {
    values: Cow<'a, [X]>
}

impl<'a, X: Clone + 'a> Items<'a, X> where [X]: ToOwned<Owned = Vec<X>> {
    fn new(items: Cow<'a, [X]>) -> Self {
        Items {
            values: items 
        }
    }
}

fn test_cow() {
    let readonly = [1,2];
    let x = &readonly[..];
    let mut borrowed = Items::new(x.into());

    match borrowed {
        Items { values: Cow::Borrowed(b)} => println!("Borrowd {b:?}"),
        _ => panic!("nothing borrowed")
    }

    //let mut clone_on_write = borrowed;
    borrowed.values.to_mut().push(3);
    println!("clone_on_write = {:?}", borrowed.values);

    // The data was mutated. Let's check it out.
    match borrowed {
        Items { values: Cow::Owned(_) } => println!("clone_on_write contains owned data"),
        _ => panic!("expect owned data"),
    }
}
