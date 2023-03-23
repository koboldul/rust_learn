use core::fmt::Debug;


pub trait Vegetable : Debug {
    fn name(&self) -> &'static str;
    
    fn cut(&self)-> &'static str {
        "cut"
    }
}

pub struct Salad<V: Vegetable> {
    pub vegetables: Vec<V>
}

pub struct GSalad {
    pub vegetables: Vec<Box<dyn Vegetable>>
}

impl<V> Salad<V> where V: Vegetable + Debug {
    pub fn make_it(&self) {
        for v in &(self.vegetables) {
            println!("{:?}", v);
            println!("{} -> {}", v.name(), v.cut());
        }
    }
}

impl GSalad {
    pub fn make_it(&self) {
        for v in &(self.vegetables) {
            println!("{:?}", v);
            println!("{} -> {}", v.name(), v.cut());
        }
    }
}

#[derive(Debug)]
pub struct Tomato {
    pub color: String
}

#[derive(Debug)]
pub struct QCumber {
    pub color: String
}

impl Vegetable for Tomato {
    fn name(&self) -> &'static str {
        "tomato"
    }
    
    fn cut(&self)-> &'static str {
        "cut tomato"
    }
}

impl Vegetable for QCumber {
    fn name(&self) -> &'static str {
        "cucumber"
    }
    
    fn cut(&self)-> &'static str {
        "cucumber slice"
    }
}