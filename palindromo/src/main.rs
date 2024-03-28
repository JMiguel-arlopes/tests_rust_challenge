use regex::Regex;

fn main() {
    let word = "arara";
    println!("resultado da palavra: {}", is_a_palindorm(word)); 
}

fn regex_string(text: &str) -> String {
    let regex =  Regex::new(r"[^a-zA-Z]").unwrap();
    return regex.replace_all(text, "").to_lowercase().to_string();
}

fn inverse_string(value: String) -> String {
    return value.chars().rev().collect();
}

fn is_a_palindorm(word: &str) -> bool {
    let reg = regex_string(word);
    let inverted_word = inverse_string(reg);
    return word == inverted_word;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_regex_string() {
        assert_eq!(regex_string("a*bC323435252532"), "abc");
    }

    #[test]
    fn test_inverse_string() {
        assert_eq!(inverse_string(("rockleevsgaara").to_string()), "araagsveelkcor");
    }

    #[test]
    fn test_is_a_palindorm() {
        let word = "arara";
        let word2 = "house";
        let word3 = "kakaroto";
        assert_eq!(is_a_palindorm(word), true);
        assert_eq!(!is_a_palindorm(word2), true);
        assert_eq!(is_a_palindorm(word3), false);
    }
}
