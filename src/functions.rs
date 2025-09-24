pub fn example_function() {
    // Write a function to get the first word from a sentence.
    let sentence = String::from("Harshit is learning rust");

    let first_word = get_first_word(sentence);

    println!("First Word is {}", first_word);
}

fn get_first_word(str: String) -> String {
    let mut word = String::new();
    for char in str.chars() {
        if char == ' ' {
            break;
        }
        word.push(char);
    }
    return word;
}