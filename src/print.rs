use std::io;

fn give_two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

fn count_characters(name: String) -> i32 {
    let mut count = 0;
    for _ in name.chars() {
        count += 1;
    }
    count
}

fn remove_vowels(name: String) -> String {
    let mut output = String::new();
    for c in name.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {}
            _ => {
                output.push(c);
            }
        }
    }
    output
}

fn print_out(str: String) {
    let devowelized_name = remove_vowels(str.clone());
    let total_characters = count_characters(str.clone());
    println!("Removing vowels yields {:?}", devowelized_name);
    println!("Total characters {:?}", total_characters);
    println!("Original string {:?}", str);
}
