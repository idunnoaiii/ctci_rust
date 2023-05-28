use std::collections::HashSet;

fn all_chars_unique(arr: &str) -> bool {
    let mut bag = HashSet::<char>::new();

    for char_item in arr.chars().into_iter() {
        let try_add = bag.insert(char_item);

        if try_add == false {
            return false;
        }
    }

    return true;
}

fn all_chars_unique_use_bitwise(arr: &str) -> bool {
    if arr.len() > 26 {
        return false;
    }

    let mut bit_field: i64 = 0;
    let a_int_char = 'a' as i64;

    for c in arr.chars() {
        let int_char = c as i64;
        let pos = int_char - a_int_char;

        if (1 << pos) & bit_field != 0 {
            return false;
        }

        bit_field |= 1 << pos;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(all_chars_unique("helo"), true);
        assert_eq!(all_chars_unique("hello"), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_chars_unique_use_bitwise("helo"), true);
        assert_eq!(all_chars_unique_use_bitwise("hello"), false);
    }
}

fn main() {
    assert_eq!(all_chars_unique("helo"), true);
    assert_eq!(all_chars_unique_use_bitwise("helo"), true);
}
