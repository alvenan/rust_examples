const MEANING_OF_LIFE:u8 = 42; //Constant value with no fixed adress
static mut z:i32 = 123; // static mut is unsafe need to created an unsafe block to compile it
fn operators() {

    //arithmetic
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = a + 1;
    println!("a + 1 = {}", a);
    a -= 2;
    println!("a - 2 = {}", a);
    //a %= 2;
    //println!("a % 2 = {}", a);

    let a_cubed = i32::pow(a, 3);
    println!("a ^ 3 = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b ^ 3 = {}, b ^ pi = {}", b_cubed, b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR & AND ^XOR ! NOR
                   // 01 OR 10 = 11 == 3
    println!("1 | 2 = {}", c);
    let two_to_10 = 1 << 10; // >> - Shift operation
    println!("2 ^ 10 = {}", two_to_10);

    // logical
    let pi_less4 = std::f64::consts::PI < 4.0; // true
                                               // > <= >= ==

    let x = 5;
    let x_is_5 = x == 5; // true
}

fn main() {
    // operators();
    println!("{}", MEANING_OF_LIFE); 
    unsafe { //unsafe block to work wit a static and mutable variable
        println!("{}", z);
        z = 1234;
        println!("{}", z);
    }

}
