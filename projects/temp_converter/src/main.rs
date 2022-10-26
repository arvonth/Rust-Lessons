use std::io;

fn main() {

    loop {
        println!("1) Fahrenheit -> Celcius?");
        println!("2) Celcius -> Fahrenheit?");
        println!("q) Quit");
        let mut conversion = String::new();
            io::stdin().read_line(&mut conversion)
                .expect("Failed to read line");
        
        //Convert response to a string and handle the quit response
        let conversion: String = conversion.trim().to_lowercase();
        if conversion == "q" {
            break;
        }

        let conversion: u32 = match conversion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if conversion == 1 {
            println!("Enter a Temperature in Degrees Fahrenheit");
        } else if conversion == 2 {
            println!("Enter a Temperature in Degrees Celcius");
        } else {
            println!("invalid entry");
            continue;
        }
        let mut degrees_in = String::new();
            io::stdin().read_line(&mut degrees_in)
                .expect("Failed to read line");

        let degrees_in: f32 = match degrees_in.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if conversion == 1 {
            let degrees_out = (degrees_in - 32.0 ) * 5.0 / 9.0;
            println!("\t{}°F is {}°C\n",degrees_in,degrees_out);
        } else if conversion == 2 {
            let degrees_out = degrees_in * 9.0 / 5.0 + 32.0;
            println!("\t{}°C is {}°F\n",degrees_in,degrees_out);
        } 
        
    }
}
