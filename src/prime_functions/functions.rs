
use std::io::stdin;


fn prompt(msg: &str) -> String{
    let mut res = String::new();
    println!("{}",msg);
    stdin().read_line(&mut res).expect("failed to read the line");
    res
}

fn is_prime(primes: &Vec<u32>, num: u32) -> Option<u32> {
    let mut prime_iter = primes.iter();
    let mut prime = prime_iter.next();
    while prime.is_some() && num % prime.unwrap() != 0 {
        prime = prime_iter.next();
    }
    if prime.is_none() {
        return Some(num)
    }
    None
}

pub fn get_number_from_user()-> u32 {

    let max = prompt("Enter the upper bound to search for: ");

    // todo, explain the ? also and introduce the Result after using Option<T>
    match max.trim_end().parse::<u32>() {
        Ok(val) => val,
        Err(e) => {
            println!("We could extract a number from your input, please try again.\n Err: {}",e);
            get_number_from_user()
        }
    }
}

pub fn add_to_vec_if_prime(primes: &mut Vec<u32>, num: u32) -> bool {
    /* Option 1
    if is_prime(&primes, num).is_some() {
        primes.push(num);
        return true;
    }
    false
     */

    match is_prime(&primes, num) {
        None => {false}
        Some(_) => {
            primes.push(num);
            true
        }
    }
}
