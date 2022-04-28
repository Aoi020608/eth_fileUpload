///Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
///
///

pub fn is_valid(s: String) -> bool {
    let brackets = vec!['(', ')', '{', '}', '[', ']'];

    let strings: Vec<char> = s.as_str().chars().collect();

    let mut c1 = true;
    let mut c2 = true;
    let mut c3 = true;

    for i in 0..strings.len() {
        if strings[i] == brackets[1] || strings[i] == brackets[3] || strings[i] == brackets[5] {
            continue;
        }
        if strings[i] == brackets[0] {
            if strings[i + 1] != brackets[1] {
                c1 = false;
            }
        }
        if strings[i] == brackets[2] {
            if strings[i + 1] == brackets[3] {
                c2 = false;
            }
        }
        if strings[i] == brackets[4] {
            if strings[i + 1] == brackets[5] {
                c3 = false;
            }
        }
    }

    c1 && c2 && c3
}

pub fn is_valid_2(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        match c {
            ')' => match stack.pop() {
                Some(bracket) => {
                    if bracket != '(' {
                        return false;
                    }
                }
                None => return false,
            },
            '}' => match stack.pop() {
                Some(bracket) => {
                    if bracket != '{' {
                        return false;
                    }
                }
                None => return false,
            },
            ']' => match stack.pop() {
                Some(bracket) => {
                    if bracket != '[' {
                        return false;
                    }
                }
                None => return false,
            },
            _ => stack.push(c),
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid_1() {
        let input = String::from("()");
        let is_ok = is_valid(input);
        assert_eq!(is_ok, true);
    }

    #[test]
    fn test_is_valid_2() {
        let input = String::from("()[]{}");
        let is_ok = is_valid_2(input);
        assert_eq!(is_ok, true);
    }

    #[test]
    fn test_is_valid_3() {
        let input = String::from("(]");
        let is_ok = is_valid_2(input);
        assert_eq!(is_ok, false);
    }
}
