use std::{borrow::Cow, usize};

pub mod macros;

const EMPTY_STR: &str = "";

pub trait OptionalAsRefStr {
    fn as_opt_str(&self) -> Option<&str>;
}

impl OptionalAsRefStr for &str {
    fn as_opt_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl OptionalAsRefStr for String {
    fn as_opt_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl OptionalAsRefStr for Option<String> {
    fn as_opt_str(&self) -> Option<&str> {
        self.as_ref().map(|s| s.as_ref())
    }
}

impl OptionalAsRefStr for Option<&str> {
    fn as_opt_str(&self) -> Option<&str> {
        self.as_ref().map(|s| s.as_ref())
    }
}

pub fn is_blank(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => value.trim().is_empty(),
        None => true,
    }
}

pub fn is_not_blank(target: impl OptionalAsRefStr) -> bool {
    !is_blank(target)
}

pub fn is_empty(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => value.is_empty(),
        None => true,
    }
}

pub fn is_not_empty(target: impl OptionalAsRefStr) -> bool {
    !is_empty(target)
}

pub fn capitalize(target: impl OptionalAsRefStr) -> String {
    match target.as_opt_str() {
        Some(value) => {
            let mut capitalized = true;
            let mut result = gen_empstring!();
            for char in value.chars() {
                if !char.is_whitespace() && capitalized {
                    result.push_str(&char.to_uppercase().to_string().as_str());
                    capitalized = false;
                } else {
                    result.push(char);
                }
            }
            result
        }
        None => gen_empstring!(),
    }
}

pub fn uncapitalize(target: impl OptionalAsRefStr) -> String {
    match target.as_opt_str() {
        Some(value) => {
            let mut uncapitalized = true;
            let mut result = gen_empstring!();
            for char in value.chars() {
                if !char.is_whitespace() && uncapitalized {
                    result.push_str(&char.to_lowercase().to_string().as_str());
                    uncapitalized = false;
                } else {
                    result.push(char);
                }
            }
            result
        }
        None => gen_empstring!(),
    }
}

pub fn substring_between<'a, S: OptionalAsRefStr, P: AsRef<str>>(
    target: &'a S,
    open: P,
    close: P,
) -> Option<&'a str> {
    match target.as_opt_str() {
        Some(value) => {
            let start = value.find(open.as_ref());
            let end = value.find(close.as_ref());
            if let Some(start_idx) = start {
                if let Some(end_idx) = end {
                    return Some(&value[start_idx..end_idx]);
                }
            }

            None
        }
        None => None,
    }
}

pub fn remove(target: impl OptionalAsRefStr, remove: impl AsRef<str>) -> String {
    match target.as_opt_str() {
        Some(value) => value.replace(remove.as_ref(), EMPTY_STR),
        None => gen_empstring!(),
    }
}

pub fn reverse(target: impl OptionalAsRefStr) -> String {
    match target.as_opt_str() {
        Some(value) => value.chars().rev().collect(),
        None => gen_empstring!(),
    }
}

pub fn abbreviate<'a, S: OptionalAsRefStr>(target: &'a S, max_width: usize) -> &'a str {
    let three_length = 3;
    match target.as_opt_str() {
        Some(value) => {
            let len = value.len();
            if max_width < len {
                &value[..max_width - three_length]
            }
        }
        None => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_blank("  "));
        assert_eq!(true, is_blank(""));
        assert_eq!(false, is_blank(" ta "));
        println!("{}", capitalize(" ass"));
        println!("{}", uncapitalize("Ass"));
        println!("{}", "2".to_uppercase());
    }
}
