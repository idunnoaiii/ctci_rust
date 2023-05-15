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

    let mut dict = HashMap::<char, i32>::new();

    for char_item in s1.chars() {
        if char_item != ' ' {
            if let Some(item) = dict.get_mut(&char_item) {
                *item -= 1;
                if *item == 0 {
                    dict.remove(&char_item);
                }
            } else {
                dict.insert(char_item, 1);
            }
        }
    }

    return dict.len() == 0 || dict.len() == 1;
}


fn main() {
    palindrome_permutation("s1");
}