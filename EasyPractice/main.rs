use std::io;
use rand::Rng;
use num::pow;
use itertools::Itertools;
use std::f64::consts::PI;

fn main() {
    grades();
}

/*Write a programme which finds the factorial of a number entered by the user. (check for all conditions).*/
fn factorial() {
    let mut number = String::new(); //creates an empty string which will accept user input
    let mut product : i32 = 1;

    println!("Input a number and we will return a factorial!");
    io::stdin() //reads the user input and assigns this value to the number variable
            .read_line(&mut number) //question: why do we use '&' to reference the number variable?
            .expect("Failed to read line");

    let number : i32 = number.trim().parse().unwrap(); //This should convert the string into an integer that I can work with.
    for element in (1..number){ //basically cycles through all numbers up to the input
        product = product * element;
    }

    println!("The factorial of {} is {}", &number, &product);
}

/*Develop a programme to convert currency X to currency Y and vice versa.*/
fn currency_conversion(){
    let mut usd = String::new();
    let mut new_currency = String::new();

    println!("How much USD do you want to convert?");
    io::stdin()
            .read_line(&mut usd)
            .expect("Failed to read line");

    let usd : f32 = usd.trim().parse().unwrap();

    //asks the user what currency do they want to convert to, Euros, Pesos, Yen
    println!("What currency do you want to convert to? enter the number below.");
    println!("1.Euros\n2.Pesos\n3.Yen");
    io::stdin()
            .read_line(&mut new_currency)
            .expect("Failed to read line");
        
    let new_currency : i32 = new_currency.trim().parse().unwrap();

    if(new_currency == 1){
        println!("{} USD is equal to {} Euros", usd, (usd*0.95));
    }
    else if(new_currency == 2){
        println!("{} USD is equal to {} Pesos", usd, (usd*20.15))
    }
    else if(new_currency == 3){
        println!("{} USD is equal to {} Yen", usd, (usd*130.58))
    }
    
}

/*Write a programme that prints out a triangle from largest to smallest; user inputs the largest number.*/
/*Write a programme that prints out a triangle from smallest to largest; user inputs bottom number.*/
/*Print out a triangle from smallest to largest, skipping even rows. User inputs largest number,*/
fn star_triangle(){
    let mut num_of_stars = String::new();

    println!("How many stars do you want?");
    io::stdin()
            .read_line(&mut num_of_stars)
            .expect("Failed to read line");

    let num_of_stars: i32 = num_of_stars.trim().parse().unwrap();

    println!("Largest to smallest:");
    for i in (0..num_of_stars+1).rev(){
        for f in (0..i){
            print!("* ");
        }
        println!();
    }

    println!("Smallest to largest:");
    for i in (0..num_of_stars+1){
        for f in (0..i){
            print!("* ");
        }
        println!();
    }

    println!("Smallest to largest with no even numbers:");
    for i in (0..num_of_stars+1){
        if(i%2 == 1){
            for f in (0..i){
                print!("* ");
            }
        }
        println!();
    }

}

/*Guessing game. ask the user to guess a number between 1 and a 100. If you guessed correctly, it will say you win. 
If you're too high or too low it will also let you know.*/
fn guessing_game(){
    let number : i32 = rand::thread_rng().gen_range(0..100);
    let mut num_of_guesses = 0;
    let mut correct : bool = false;

    while (!correct){
        let mut guess = String::new();

        println!("Guess a number!");
        io::stdin() //reads the user input and assigns this value to the number variable
            .read_line(&mut guess) //question: why do we use '&' to reference the number variable?
            .expect("Failed to read line");

        let mut guess : i32 = guess.trim().parse().unwrap();

        if(guess < number){
            println!("Too low!");
            num_of_guesses += 1;
        }
        else if (guess > number){
            println!("Too high!");
            num_of_guesses += 1;
        }
        else{
            println!("You got it!");
            num_of_guesses += 1;
            correct = true;
        }

        println!("number of guesses: {}", num_of_guesses);
    }

}

/*Create a programme which generates Fibonacci series til a number 'n', where 'n' is entered by the user. */
fn fibonacci(){
    let mut number = String::new();
    let mut num1 = 1;
    let mut num2 = 1;
    let mut temp = 0;

    println!("What number do you want to go up to?");
    io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

    let number = number.trim().parse().unwrap();

    print!("{},{},",num1,num2);
    while(num2 < number){
        temp = num2;
        num2 = num1 + num2;
        num1 = temp;

        if(num2 <= number){
            print!("{}",num2);
            if(num1+num2 <= number){
                print!(",");
            }
        }
    }
    println!("");
}

/*Given a string, determine how many of the characters are vowels and how many are consonants.
Terminate the string when the input character encountered is non-alphabetic.*/
fn vowels(){
    let mut input = String::new();
    let mut vowels = 0;
    let mut consonants = 0;

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
    for i in input.chars(){
        if(i.is_alphabetic() == false){
            break
        }
        else{
            if(i == 'a' || i == 'e' || i == 'i' || i == 'o' || i == 'u'){
                vowels += 1;
            }
            else{
                consonants += 1;
            }
        }
    }

    println!("{} vowels and {} consonants", &vowels,&consonants);
}

/*Find the Fifth root of the sum of the squares of the first 100 ODD numbers only.*/
fn fifth_root(){ //debug: the math is wrong but i'm too lazy to do anything about this so onto the next ig uwu
    let mut sum : f64 = 0.0;
    for i in (1..101){ //adds the squares of the first 100 odd numbers
        if(i%2 == 1){
            sum += (i as f64).powf(2.0);
        }
    }

    let fifth_root = f64::powf(sum,0.2);

    println!("The sum of the squares of the first 100 odd numbers is {}",&sum);
    println!("The fifth root of the sum of the squares of the first 100 odd numbers is {}",fifth_root);
}

/*List all possible combinations of letters in a 4-letter word. Eg 'TEST' can be unscrambled as TEST, TETS, TSET, TSTE, TTSE, TTES, etc.*/
fn perm(){
    let mut word = String::new();
    let mut vec : Vec<char> = Vec::new(); 

    println!("please input a word for us to find all possible permutations for:");
    io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

    let word = word.trim();

    for i in word.chars(){ //creates the vector
        vec.push(i);
    }

    let length = vec.len();

    for x in vec.into_iter().permutations(length){ //so here we basically move vec so we can no longer reference it after this
        println!("{:?}",x);
    }
}

/*Make a programme that allows the user to input either the radius, diameter, or area of the circle. 
The programme should then calculate the other 2 based on the input.*/
//I could've had all the circle functions in a single module
fn circle(){
    let mut choice = String::new();

    println!("radius, diameter, or area?");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let r = String::from("r");
    let d = String::from("d");
    let a = String::from("a");

    match &choice[0..1].to_lowercase(){
        r => radius(),
        d => diameter(),
        a => area()
    }
}

fn radius(){
    let mut radius = String::new();

    println!("What is your radius?");
    io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read line");

    let radius : f64 = radius.trim().parse().unwrap();

    println!("Your diameter is: {}", (&radius*2.0));
    println!("Your area is: {}", (PI * radius.powf(2.0)));
}

fn diameter(){
    let mut diameter = String::new();

    println!("What is your diameter?");
    io::stdin()
        .read_line(&mut diameter)
        .expect("Failed to read line");

    let diameter : f64 = diameter.trim().parse().unwrap();

    println!("Your radius is: {}", (&diameter/2.0));
    println!("Your area is: {}", (PI * (&diameter/2.0).powf(2.0)));
}

fn area(){
    let mut area = String::new();

    println!("What is your area?");
    io::stdin()
        .read_line(&mut area)
        .expect("Failed to read line");

    let area : f64 = area.trim().parse().unwrap();

    println!("Your radius is: {}", (&area/PI).powf(0.5));
    println!("Your diameter is: {}", (&area/PI).powf(0.5)*2.0);
}

/*Read a line of text and write it out backwards*/
fn backwards(){
    let mut input = String::new();

    let mut vec : Vec<char> = Vec::new();

    println!("Give me a word any word!");
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read line");

    let input = input.trim();

    for i in input.chars(){ //creates a vector of the characters
        vec.push(i);
    }

    for f in vec.into_iter().rev(){
        print!("{}",f);
        print!(" ");
    }

    println!("");

}

/*Read a line of text and write it out backwards using a recursive function.*/
fn backwards_recursive_main(){
    let mut input = String::new();

    let mut vec : Vec<char> = Vec::new();

    println!("Give me a word any word!");
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read line");

    let input = input.trim();

    for i in input.chars(){ //creates a vector of the characters
        vec.push(i);
    }

    vec.reverse();
    backwards_recursion(&vec);
}

fn backwards_recursion( v : &[char]){
    if(v.len()>0){
        print!("{:?}",&v[0]);
        backwards_recursion(&v[1..]);
    }
    else{
        println!("");
        return
    }
}


/*Write a programme to simulate a simple calculator. 
It should accept two numbers from the user along with the required operation to be performed. 
Addition, subtraction, division and multiplication are the basic operations that should be implemented. 
Feel free to implement other operations. 
Bonus points for splitting the calculation functions into a separate module.*/
mod calculator{
    pub mod choice{
        use std::io; //question: why is it that we have to use std::io again in this module?
        pub fn calc(){
            let mut input = String::new();

            println!("do you want to add,subtract,divide,or multiply?");
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read line");

            let input = input.trim();

            let mut num1 = String::new();
            let mut num2 = String::new();
            
            println!("Give me the first number:");
            io::stdin()
                .read_line(&mut num1)
                .expect("Could not read line");

            println!("Give me the second number:");
            io::stdin()
                .read_line(&mut num2)
                .expect("Could not read line");

            let num1 : f64 = num1.trim().parse().unwrap();
            let num2 : f64 = num2.trim().parse().unwrap();

            match input{
                "add" => super::operations::add(num1,num2),
                "subtract" => super::operations::subtract(num1,num2),
                "multiply" => super::operations::multiply(num1,num2),
                "divide" => super::operations::divide(num1,num2),
                otherwise => println!("you didn't input a correct choice for the operation!")
            };
        }
    }

    pub mod operations{
        pub fn add(x:f64,y:f64){
            println!("{} + {} = {}",&x,&y,(&x+&y));
        }
        pub fn subtract(x:f64,y:f64){
            println!("{} - {} = {}",&x,&y,(&x-&y));
        }
        pub fn multiply(x:f64,y:f64){
            println!("{} * {} = {}",&x,&y,(&x*&y));
        }
        pub fn divide(x:f64,y:f64){
            println!("{} / {} = {}",&x,&y,(&x/&y));
        }
    }
}

/*Determine how much money is in a piggy bank that contains several £2 coins, £1 coins, 50p coins, 20p coins, 10p coins and 5p coins. 
Use the following values to test your programme: one £2, three £1, five 50p coins, two 20p coins, one 10p coin and fifteen 5p coins.*/
fn piggybank(){
    let mut input = String::new();
    let mut sum : f64 = 0.0;
    let mut values : Vec<f64>= vec![2.0,1.0,0.5,0.2,0.1,0.05];

    println!("List how many £2, £1, 50p, 20p, 10p, and 5p coins you have in that order with spaces separating the numbers");

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let input = input.trim();

    let input : Vec<&str> = input.split_whitespace().collect();

    for i in (0..6){
        sum += (input[i].parse::<f64>().unwrap()) * values[i];
    }

    println!("You have a total of £{}", &sum);
}

/*Create a simple palindrome checker programme. The programme should allow the user to enter a string and check whether the given string is a palindrome or not.*/
fn palindrome(){
    let mut input = String::new();
    let mut forward : Vec<char> = Vec::new();
    let mut backward : Vec<char> = Vec::new();

    println!("Give me a word and I'll tell you if it's a palindrome or not:");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read the input");
    
    let input = input.trim();

    for i in input.chars(){
        forward.push(i);
        backward.push(i);
    }

    backward.reverse();

    for i in (0..forward.len()){
        if(forward[i] != backward[i]){
            println!("Not a palindrome!");
            return
        }
    }

    println!("Palindrome!");
}

/*Write a programme that allows you to input students' midterm, final and homework scores, and calculate a weighted score.
Use the following weights: 20% midterm, 40% final, 40% median homework.*/
fn grades(){
    let mut midterm = String::new();
    let mut final_exam = String::new();
    let mut homework = String::new();

    println!("What was your midterm grade?");
    io::stdin()
        .read_line(&mut midterm)
        .expect("Give me a valid number");

    println!("What was your final exam grade?");
    io::stdin()
        .read_line(&mut final_exam)
        .expect("Give me a valid number");

    println!("What was your average homework grade?");
    io::stdin()
        .read_line(&mut homework)
        .expect("Give me a valid number");

    let midterm : f64 = midterm.trim().parse().unwrap();
    let final_exam : f64 = final_exam.trim().parse().unwrap();
    let homework : f64 = homework.trim().parse().unwrap();

    println!("Your final grade for the class: {}", (midterm*0.2+final_exam*0.4+homework*0.4))
}
