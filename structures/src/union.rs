union IntOrFloat {
    i: i32,
    f: f32,
}
fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("Meaning of Life!")
            }
            IntOrFloat { f } => {
                println!("value is {}", f);
            }
        }
    }
}
pub fn union() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { i: 5 });
}
