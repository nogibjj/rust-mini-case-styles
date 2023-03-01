use std::io;

fn main() {
    // Get user input
    let words = get_user_input("Enter words (separated by spaces): ");
    let case_type = get_user_input("Enter case type (snake/camel/pascal): ");

    // Generate connected string
    let connected_string = match case_type.as_str() {
        "snake" => generate_snake_case(&words),
        "camel" => generate_camel_case(&words),
        "pascal" => generate_pascal_case(&words),
        _ => panic!("Invalid case type"),
    };

    // Display output
    println!("Connected String: {}", connected_string);
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn generate_snake_case(words: &str) -> String {
    words.replace(" ", "_").to_lowercase()
}

fn generate_camel_case(words: &str) -> String {
    let words_vec: Vec<&str> = words.split(" ").collect();
    let mut result = String::new();

    for (i, word) in words_vec.iter().enumerate() {
        if i == 0 {
            result.push_str(word);
        } else {
            let capitalized_word = word[..1].to_uppercase() + &word[1..];
            result.push_str(&capitalized_word);
        }
    }

    result
}

fn generate_pascal_case(words: &str) -> String {
    let words_vec: Vec<&str> = words.split(" ").collect();
    let mut result = String::new();

    for word in words_vec.iter() {
        let capitalized_word = word[..1].to_uppercase() + &word[1..];
        result.push_str(&capitalized_word);
    }

    result
}

#[test]
fn test_generate_camel_case() {
    let input = "hello world";
    let output = generate_camel_case(input);
    assert_eq!(output, "helloWorld");

    let input = "hello";
    let output = generate_camel_case(input);
    assert_eq!(output, "hello");

    let input = "world";
    let output = generate_camel_case(input);
    assert_eq!(output, "world");
}

#[test]
fn test_generate_pascal_case() {
    let input = "hello world";
    let output = generate_pascal_case(input);
    assert_eq!(output, "HelloWorld");

    let input = "hello";
    let output = generate_pascal_case(input);
    assert_eq!(output, "Hello");

    let input = "world";
    let output = generate_pascal_case(input);
    assert_eq!(output, "World");
}
