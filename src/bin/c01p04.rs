use std::collections::HashMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(palindrome_permutation("tact coa"), true);
    }

    #[test]
    fn test2() {
        assert_eq!(palindrome_permutation("tacto coa"), true);
    }

    #[test]
    fn test3() {
        assert_eq!(palindrome_permutation("tactl coa"), false);
    }
}


fn palindrome_permutation(s1: &str) -> bool {

    let mut dict = HashMap::<String, i32>::new();

    for char_item in s1.chars() {
        if char_item == ' ' {
            continue;
        }

        let temp_char = char_item.to_lowercase().to_string();

        if let Some(item) = dict.get_mut(&temp_char) {
            *item -= 1;

            if *item == 0 {
                dict.remove(&temp_char);
            }
        } else {
            dict.insert(temp_char, 1);
        }
    }

    return dict.len() == 0 || dict.len() == 1;
}


fn main() {
    palindrome_permutation("s1");
}