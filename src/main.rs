use std::io;

fn main() {
    let mut start_unit = String::new();
    let mut degrees = String::new();

    println!("Temperature Converter!");
    println!("Input Units °(F/C): ");
    io::stdin()
        .read_line(&mut start_unit)
        .expect("Failed to read input_unit");

    println!("Input the degrees");
    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read input_degrees");

    let degrees: f64 = degrees 
        .trim()
        .parse()
        .expect("Failed to parse degrees into f64");

    start_unit = start_unit.trim().to_ascii_uppercase();
    if start_unit == "C" {
        let converted = celsius_to_farenheit(degrees);
        println!("{}°C in Farenheit is {:.2}°F", degrees, converted);
    } else if start_unit == "F" {
        let converted = farenheit_to_celsius(degrees);
        println!("{}°F in Celius is {:.2}°C", degrees, converted);
    }

}

fn farenheit_to_celsius(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

fn celsius_to_farenheit(c: f64) -> f64 {
    return c * 9.0 / 5.0 + 32.0
}