#![allow(dead_code)]
use std::mem;
struct Point {
        x: f64,
        y: f64,
        z: f64,
}

fn origin() -> Point {
        Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
        }
}

pub fn stack_heap() {
        let p1 = origin(); //This will save 3 variables f64 so 24 bytes
        let p2 = Box::new(origin()); //This will save the address of 8 bytes for the structure

        println!("p1 takes up {} bytes", mem::size_of_val(&p1));
        println!("p2 takes up {} bytes", mem::size_of_val(&p2));

        let p3 = *p2;

        println!("p23 takes up {} bytes", mem::size_of_val(&p3));
}
