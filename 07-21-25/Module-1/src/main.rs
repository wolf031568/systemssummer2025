fn main() {
    let original_temperature : f64 = 25.75;
    println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(original_temperature));
    println!("Celsius to Fahrenheit {}", celsius_to_fahrenheit(original_temperature));
    number_analyzer();
    guessing_game();
    println!("Program ended.");
}
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - 32.0) * (5.0/9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * (9.0/5.0)) + 32.0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn number_analyzer(){
    let nums: [i32; 10] = [28, 55, 79, 80, 99, 21, 34, 67, 90, 8];

    for &num in &nums {
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

fn guessing_game(){
let secret: i32 = 21;
let mut attemps = 0;
let guesses: [i32; 6] = [50, 10, 75, 20, 30, 21];
for &guess in &guesses {
    attemps += 1;
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