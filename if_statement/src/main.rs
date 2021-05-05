fn if_statement() {
    let temp = 40;
    if temp > 30 {
        println!("Really Hot outside!");
    } else if temp < 10 {
        println!("Really cold outside");
    } else {
        println!("The temperature is ok!");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    println!(
        "is it {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );

    println!(
        "is it {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "OK"
        }
    );
}

fn main() {
    if_statement();
}
