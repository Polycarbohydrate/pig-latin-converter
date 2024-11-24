pub fn appending(input: String) {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let formated = input.replace("\r\n","");
    let constant_word = "ay ";
    let vowel_word = "hay ";
    let words: Vec<String> = formated
        .split_whitespace()
        .map(|word| {
            // Get the first character of the word
            let first_char = word.chars().next().unwrap_or('\0');

            if vowels.contains(&first_char) {
                // If the first character is a vowel, keep the word as is
                format!("{}{}", word, vowel_word)
            } else {
                // If the first character is a consonant, append it and the suffix
                let rest_of_word: String = word.chars().skip(1).collect();
                format!("{}{}{}", rest_of_word, first_char, constant_word)
            }
        })
        .collect();
    let result = words.join("");
    println!("======================================================================================");
    println!(" ");
    println!("{}", result);
}