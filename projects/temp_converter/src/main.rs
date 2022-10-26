use std::io;

fn main() {

    loop {
        println!("Enter a Temperature in Degrees Fahrenheit");
        let mut degrees_f = String::new();
            io::stdin().read_line(&mut degrees_f)
                .expect("Failed to read line");

            let degrees_f: f32 = match degrees_f.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let degrees_c = (degrees_f - 32.0 ) * 5.0 / 9.0;
            println!("{} degrees Fahrenheit is {} Celcius",degrees_f,degrees_c);
    }
}
