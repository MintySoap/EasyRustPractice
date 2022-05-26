#![allow(unused_parens)]
use std::io;
use primes;

/* Problem :
Write a programme which will print all the pairs of prime numbers whose sum equals the number entered by the user. 
Eg 10 = 5 + 5, 7 + 3; 12 = 11 + 1, 5 + 7
*/

fn main() {
    let mut number = String::new();
    println!("Howdy! Please input a number, and we'll tell you all of the pairs of prime numbers that add up to that number");

    io::stdin()
        .read_line(&mut number)
        .expect("ERROR");

    let number : i32 = number.trim().parse().unwrap();
    prime_pairs(number);
}

fn prime_pairs(num : i32){
    let mut primes : Vec<i32> = Vec::new();
    let mut prime_pairs : Vec<(i32,i32)> = Vec::new();

    //1. Find all of the prime numbers from 2 up to the given number
    for i in (2..num){ 
        if (primes::is_prime(i as u64)){
            primes.push(i);
        }
    }

    //2. Cycle through the list of prime numbers using nested for loops to find all the possible pairs.
    for f in &primes{
        for g in &primes{
            if (f+g == num){
                //3. Store each pair in a tuple and then store the tuples in a vector
                if(!prime_pairs.contains(&(*g,*f)) && f != g){
                    prime_pairs.push((*f,*g));
                }
            }
        }
    }

    //4. Print the vector at the end for the user
    for (x,y) in prime_pairs{
        println!("{} + {} = {}",x,y,num);
    }
}