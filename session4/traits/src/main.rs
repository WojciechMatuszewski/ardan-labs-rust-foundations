use std::fmt;
use std::fmt::{Debug, Formatter};

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

trait Animal: Debug {
    fn speak(&self);
}

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
    println!("{animal:?}");
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("meow")
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof")
    }
}

fn main() {
    let cat = Cat;

    speak_twice(&cat);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
}
