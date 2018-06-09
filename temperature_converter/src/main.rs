use std::io;

fn main() {
    println!("Temperature in C or F (e.g. 23 C):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    
    let is_celcius = input.ends_with("C");
    let is_fahrenheit = input.ends_with("F");

    let to_trim: &[_] = &['C', 'F', ' '];
    let temperature = input.trim_matches(to_trim);

    let temperature: f64 = temperature.parse()
        .expect("Please enter a number like 23 C or 97 F");

    if is_celcius {
        let fahrenheit = celcius_to_fahrenheit(temperature);
        println!("-> {} F", fahrenheit);
    } else if is_fahrenheit {
        let celcius = fahrenheit_to_celcius(temperature);
        println!("-> {} C", celcius);
    } else {
        panic!("I don't know if you entered Celcius or Fahrenheit!");
    }
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0  + 32.0
}
