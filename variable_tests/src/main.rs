#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem; //package to check some info from memory

fn main() {
    //INTERGERS

    let a: u8 = 250; // u8 = usigned 8bits
    println!("a = {}", a); // immutable varible
                           //a = 5 // This cause compilation fail

    let mut b: i8 = 51; // i8 = signed 8bits
                        // mut = turns the variable mutable
    println!("before b = {}", b);

    b = 111;
    println!("after b = {}", b);

    let mut c = 1; // Rust compile will find the type
                   // This example will create an i32 (signed integer with 32 bits, 8 bytes)

    println!("c = {}, {} bytes", c, mem::size_of_val(&c)); //mem:: is a lib to check some memory informations

    c = -1;
    println!("c = {}", c);

    // u8, u16, u32, u64, i8, i16, i32, i64

    // USIZE ISIZE - This use the OS lenght, my case 64-bits

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, {} bytes, {}-bit", z, size_of_z, size_of_z * 8);

    //CHAR
    let d: char = 'x';
    println!("\'{}\' = a char, size = {} bytes", d, mem::size_of_val(&d));

    // FLOAT POINT f32 f64

    let e: f64 = 2.5;
    println!("e = {}, {} bytes", e, mem::size_of_val(&e));

    //BOOLEAN

    let g: bool = false; // true
    println!("g = {}, {} bytes", g, mem::size_of_val(&g));


}
