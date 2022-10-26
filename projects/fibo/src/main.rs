use std::io;

fn main() {

    loop {
        println!("Enter N for the Fibonacci sequence or q");
        let mut fibo_n = String::new();
            io::stdin().read_line(&mut fibo_n)
                .expect("Failed to read line");
        
        //Convert response to a string and handle the quit response
        let fibo_n: String = fibo_n.trim().to_lowercase();
        if fibo_n == "q" {
            break;
        }

        let fibo_n: u32 = match fibo_n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //render the sequence using a loop
        let mut t1 = 0;
        let mut t2 = 1;
        let mut next_term = t1 + t2;
        print!("\n\tFibonacci Series: {} {} ",t1,t2);
        for _number in 2..fibo_n+1 {
            print!("{} ",next_term);
            t1 = t2;
            t2 = next_term;
            next_term = t1 + t2;
        }
        println!("")




        
    }
}
