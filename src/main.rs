mod prime_functions;

/*
This is a code exercise for le Club Robot de INSA toulouse.
The Objective is to make a bigger problem that the proposed exercices and teach a lot of the
basic "wired" concepts of rust:
    - non mutable variables
    - what are macros (not how to use them)
    - type inference
    - variable shadowing
    - borrow-checker
    - modules
    - private modules by default
    - u32 vs i32
    - Option and Result
    - match
    - turbo-fish operator (parse::<u32>) (https://turbo.fish/)
    - iterators
 */


fn main() {
    let max: u32 = prime_functions::functions::get_number_from_user();
    println!("Searching primes until {}", max);

    let mut primes: Vec<u32> = vec![2]; // base known primes excluding 1;

    for i in 2..max {
        prime_functions::functions::add_to_vec_if_prime(&mut primes, i);
    }

    println!("Found {} primes in {} num", primes.len(), max);
    println!("Biggest prime: {}", primes.last().unwrap());
}










