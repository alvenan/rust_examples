fn scope_shadowing() {
    let a = 123;
    
    {
        let b =456;
        println!("inside, b = {}", b);

        let a = 321;
        println!("inside, a = {}", a);
    }

    //println!("inside, b = {}", b); //b is out of scope here
    println!("outside, a = {}", a);
}

fn main() {
    scope_shadowing();
}
