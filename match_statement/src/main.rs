fn main() {
    let country_code = 46;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };
    println!("The country with code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        false => "no"
    };
    println!("s is {}", s);

}
