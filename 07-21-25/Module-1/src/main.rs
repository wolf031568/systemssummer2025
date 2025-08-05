fn main() {
    let original_temperature : f64 = 25.75;
    println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(original_temperature));
    println!("Celsius to Fahrenheit {}", celsius_to_fahrenheit(original_temperature));
    number_analyzer();
    guessing_game();
    println!("Program ended.");
}

//Assignment 1: Temperature Converter
fn fahrenheit_to_celsius(f: f64) -> f64{
    //Celsius -> Fahrenheit conversion formula: C = (F - 32) * (5/9)
    (f - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    //Fahrenheit -> Celsius conversion formula: F = (C * (9/5)) + 32
    (c * (9.0/5.0)) + 32.0
}

fn is_even(n: i32) -> bool {
    //Returns true if number is even, false if odd
    n % 2 == 0
}

//Assignment 2: Number Analyzer
fn number_analyzer(){
    //Rand numbers for testing
    let nums: [i32; 10] = [28, 55, 79, 80, 99, 21, 34, 67, 90, 8];
    //For loop to go through each of the numbers
    for &num in &nums {
        //if-else statements to assign an output based on the number
        if num % 15 == 0 {
            println!("{}: FizzBuzz", num);
        } 
        else if num % 3 == 0 {
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0 {
            println!("{}: Buzz", num);
        }
        else if is_even(num) {
            println!("{}: Even", num);
        }
        else{
            println!("{}: Odd", num);
        }
    }
    //initialize the sum and index that will be used to stop the while loop
    let mut sum: i32 = 0;
    let mut i = 0;
    //while loop to find the sum of the numbers
    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }
    println!("Sum of numbers: {}", sum);
    let mut largest: i32 = nums[0];
    //for loop to find the largest number 
    for &n in &nums {
        if n > largest {
            largest = n;
        }
    }
    println!("Largest number: {}", largest);
}

//Assignment 3: Guessing Game
fn guessing_game(){
let secret: i32 = 21;
let mut attemps = 0;
let guesses: [i32; 6] = [50, 10, 75, 20, 30, 21];
for &guess in &guesses {
    attemps += 1;
    //simple match statement to check each guess with the secret number.
    match check_guess(guess, secret){
        0 => {
            println!("Guess {} is correct. Attemps: {}", guess, attemps);
            break;
        }
        1 => {
            println!("Guess {} is too high. Attemps: {}", guess, attemps);
        }
        -1 => {
            println!("Guess {} is too low. Attemps: {}", guess, attemps);
        }
        _ => {
            println!("Invalid guess: {}", guess)
        }
    }
}
}
//Checks the guess against the secret number (Separated for clarity)
fn check_guess(guess : i32, secret : i32) -> i32 {
    if guess == secret {
        0
    }
    else if guess > secret {
        1
    }
    else{
        -1
    }
}