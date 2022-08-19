pub fn strings() {
    let word = String::from("okkamelon");
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let first_letter = word.chars().nth(0).unwrap();

    if vowels.contains(&first_letter) {
        let in_pig_latin = format!("{}-hay", word);
        println!("{} in pig latin is: {}", word, in_pig_latin);
    } else {
        // extracting chars from initial word
        let mut word_chars = word.chars();
        // removing the first char
        word_chars.next();
        // composing a new word in pig latin
        let in_pig_latin = format!("{}-{}ay", word_chars.as_str(), first_letter);
        println!("{} in pig latin is: {}", word, in_pig_latin);
    }
}
