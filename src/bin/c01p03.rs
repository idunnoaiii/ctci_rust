fn urlify<'a>(url: &'a str) -> String {
    let mut bag = Vec::<String>::new();

    for (idx, chr) in url.char_indices() {
        if chr == ' ' && (idx < url.len() - 1 && url.chars().nth(idx + 1).unwrap() != ' ') {
            bag.push("%20".to_string());
        } else {
            if chr != ' ' {
                bag.push(chr.to_string());
            }
        }
    }

    return bag.join("").as_str().to_string();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify("Mr John Smith    "), "Mr%20John%20Smith");
    }
}

fn main() {
    println!("{}", urlify("Mr John Smith    "));
}
