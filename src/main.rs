use std::io::Write;

fn prompt<T: std::str::FromStr>(message: &str) -> T {
    let mut input = String::new();
    loop {
        print!("{message}");
        let _ = std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        match input.trim().parse::<T>() {
            Ok(value) => {
                return value;
            }
            Err(_) => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}

fn get_fib(n: &i32) -> i64 {
    pub const PHI: f64 = 1.618033988749894848204586834365638118_f64;

    return (PHI.powi(*n) / 5_f64.sqrt()).round() as i64;
}

fn main() {
    let x = prompt::<i32>("Enter a number: ");

    let fib = get_fib(&x);

    println!("{fib}");
}
