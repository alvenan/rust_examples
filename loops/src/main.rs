fn while_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue; // continue command makes the loop start go to begining
        } // in this case the print will not run when x = 64

        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        //while true
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() { // .enumerate gives two values 1st the interaction number
        println!("{}: {}", pos, y);        // and 2nd the array value
    }
}

fn main() {
    //    while_loop();
    for_loop();
}
