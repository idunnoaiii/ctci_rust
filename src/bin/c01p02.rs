use std::collections::HashMap;

fn is_permutation(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut char_counter = HashMap::<char, i32>::new();

    for c in str1.chars() {
        let _ = char_counter.insert(c, char_counter.get(&c).unwrap_or(&0) + 1);
    }

    for c in str2.chars() {
        if let Some(x) = char_counter.get_mut(&c) {
            if *x == 0 {
                return false;
            }

            *x -= 1;
        }

        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}
}

fn main() {
    println!("{}", is_permutation("hello", "hello"));
    println!("{}", is_permutation("hello", "olhle"));
    println!("{}", is_permutation("hello", "olhla"));
}
