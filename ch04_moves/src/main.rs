use std::rc::Rc;

fn main() {
    test_rc();
}

fn test_box() {
    let a = Box::new((0.625, 0.5));
    let label = format!("{:?}", a);

    assert_eq!(label, "(0.625, 0.5)");
    print!("{}", label);
}

fn test_box_ref() {
    let x = "xsde".to_string();
    let f = x;
    let a = Box::new(f);
    let b = a.as_ref();
    //let c = &f;

    print!("{}", a);
    //print!("{}", *c);
}

fn test_rc() {
    let x = "xxx".to_string();
    let a = Rc::new(x);

    take_rc_1(a.clone());
    take_rc_2(a.clone());

    println!("{}", a);
}

fn test_vec_mv() {
    let mut ns = Vec::new();
    for i in 0..10 {
        ns.push(i.to_string());
    }

    let third = std::mem::replace(&mut ns[2], "substitute".to_string());
    print_mv(third);

    for x in ns {
        print_mv(x);
    }
    
    //let x = ns[0];
    //let x2 = ns[1];
}

fn take_rc_1(a: Rc<String>) {
    println!("1: {}", a);
}

fn take_rc_2(a: Rc<String>) {
    println!("2: {}", a);
}

fn print_mv(s:String){
    println!("{}", s);
}

#[test]
fn test_rc_test() {
    test_rc()
}

#[test]
fn test_vec_mv_test() {
    test_vec_mv()
}
