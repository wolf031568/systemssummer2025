fn main() {
    get_step_result();
    word_frequency_counter();
    println!("Program ended.");
}

//Assignment 1: Mutable Reference Sum with Step
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    //borrowed total as a mutable reference.
    let mut i = low; //index the starting point
    while i <= high {
        *total += i; //dereferences total since it is a mutable reference, to be able to modify it
        i += step;
    }
}
fn get_step_result(){
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}

//Assignment 2: Word Frequency Counter
fn most_frequent_word(text: &str) -> (String, usize) {
    //We borrow the string slice, so we dont take ownership
    //use vectors to store words and their counts (since the dataset is dynamic)
    let mut words : Vec<String> = Vec::new();
    let mut counts : Vec<usize> = Vec::new();

    //split the text into words
    for word  in text.split_whitespace(){
    let mut found = false;
    //iterate through the words vector to look for the word
    //if found, the count is incremented
    for i in 0..words.len() {
        if words[i] == word {
            counts[i] += 1; 
            found = true;
            break;
        }
    }

    //If the word wasnt there, add it to the vector
    if !found {
        words.push(word.to_string());
        counts.push(1); 
    }
}
    //Find the index of the most frequent word
    let mut max_index = 0;
    for i in 1..counts.len() {
        if counts[i] > counts[max_index] {
            max_index = i;
        }
    }   
    //return as a clone because we cant retturn a reference to a local variable                
    (words[max_index].clone(), counts[max_index])                                                          
}

fn word_frequency_counter() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}