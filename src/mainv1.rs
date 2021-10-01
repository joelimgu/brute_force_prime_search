
use std::io::stdin;


/*
This is the pure algorithm and not the final not clean code
*/
fn main() {
    let mut max = String::new();

    // todo wrap the function to introduce the ? and make it return a &str ( introduce concept of &str and String )
    println!("Enter the upper bound to search for: ");
    stdin().read_line(&mut max).expect("failed to read the line");

    // todo make a loop until not panic
    let max: u32 = match max.trim_end().parse::<u32>() {
        Ok(val) => {
            println!("Searching primes until {}", val);
            val
        }
        Err(e) => {panic!("Unable to parse the your string: {}", e)}
    };

    // todo make a lot of abstraction layers
    let mut primes: Vec<u32> = vec![2];
    for i in 2..max {
        let mut prime_iter = primes.iter();
        let mut prime = prime_iter.next();
        while prime.is_some() && i % prime.unwrap() != 0 {
            prime = prime_iter.next();
        }
        if prime.is_none() {
            primes.push(i);
        }
    }
    println!("Found {}primes in {} num", primes.len(), max);
    println!("Biggest prime: {:?}", primes.last().unwrap());
}
