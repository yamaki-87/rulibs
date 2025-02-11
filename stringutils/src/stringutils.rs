use crate::{charutils, gen_empstring};

const EMPTY_STR: &str = "";
const ABBREVIATE_MARKER: &str = "...";

pub trait OptionalAsRefStr {
    fn as_opt_str(&self) -> Option<&str>;
}

impl OptionalAsRefStr for &str {
    fn as_opt_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl OptionalAsRefStr for str {
    fn as_opt_str(&self) -> Option<&str> {
        Some(self)
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

trait CharCount {
    fn count_chars(&self) -> usize;
}

impl CharCount for str {
    fn count_chars(&self) -> usize {
        self.chars().count()
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

pub fn is_numeric(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut result = true;
            for char in value.chars() {
                if !char.is_numeric() {
                    result = false;
                }
            }

            result
        }
        None => false,
    }
}

pub fn is_alpha(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut result = true;
            for char in value.chars() {
                if !char.is_alphabetic() {
                    result = false;
                }
            }
            result
        }
        None => false,
    }
}

pub fn is_alpha_space(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut result = true;
            for char in value.chars() {
                if !char.is_alphabetic() && !char.is_whitespace() {
                    result = false;
                }
            }
            result
        }
        None => false,
    }
}

pub fn is_alpha_numeric(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut result = true;
            for char in value.chars() {
                if !char.is_numeric() && !char.is_alphabetic() {
                    result = false;
                }
            }

            result
        }
        None => false,
    }
}

pub fn is_numeric_space(target: impl OptionalAsRefStr) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut result = true;
            for char in value.chars() {
                if !char.is_numeric() && !char.is_whitespace() {
                    result = false;
                }
            }

            result
        }
        None => false,
    }
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

pub fn substring_before(
    target: impl OptionalAsRefStr,
    seperator: impl AsRef<str>,
) -> Option<String> {
    match target.as_opt_str() {
        Some(value) => {
            let start = value.find(seperator.as_ref());
            if let Some(start_idx) = start {
                return Some(value[..start_idx].to_string());
            }
            Some(value.to_string())
        }
        None => None,
    }
}

pub fn substring_after(
    target: impl OptionalAsRefStr,
    seperator: impl AsRef<str>,
) -> Option<String> {
    match target.as_opt_str() {
        Some(value) => {
            let start = value.find(seperator.as_ref());

            if let Some(mut start_idx) = start {
                start_idx += 1;
                if start_idx > value.len() {
                    return Some(value.to_string());
                }
                return Some(value[start_idx..].to_string());
            }
            Some(value.to_string())
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

pub fn abbreviate<S: OptionalAsRefStr>(target: S, max_width: usize) -> String {
    let three_length = 3;
    let mut result = gen_empstring!();
    if max_width <= three_length {
        return result;
    }

    match target.as_opt_str() {
        Some(value) => {
            let len = value.len();
            if max_width < len {
                result = value[..max_width - three_length].to_string();
                result.push_str(ABBREVIATE_MARKER);
            }
        }
        None => {}
    };

    result
}

pub fn swap_case(target: impl OptionalAsRefStr) -> String {
    let mut result = gen_empstring!();

    match target.as_opt_str() {
        Some(value) => {
            for char in value.chars() {
                if char.is_uppercase() {
                    result.extend(char.to_lowercase());
                } else if char.is_lowercase() {
                    result.extend(char.to_uppercase());
                } else {
                    result.push(char);
                }
            }
        }
        None => {}
    };

    result
}

pub fn delete_whitespace(target: impl OptionalAsRefStr) -> String {
    match target.as_opt_str() {
        Some(value) => value.chars().filter(|s| !s.is_whitespace()).collect(),
        None => gen_empstring!(),
    }
}

pub fn substring(target: impl OptionalAsRefStr, start: usize, len: usize) -> String {
    match target.as_opt_str() {
        Some(value) => value.chars().skip(start).take(len).collect(),
        None => gen_empstring!(),
    }
}

/// ## Summary
/// 入力文字列の左端から指定された文字数を抽出して返す。
/// 入力が None の場合は None を返す
///
/// ## Parameters
/// - `target`:
/// - `len`:
///
/// ## Returns
///
/// ## Examples
///```
/// use stringutils::stringutils::*;
///
/// let opt:Option<String> = None;
///
/// assert_eq!(None,left(opt,1));
/// assert_eq!(Some("".to_string()),left("abc",0));
/// assert_eq!(Some("ab".to_string()),left("abc",2));
/// assert_eq!(Some("abc".to_string()),left("abc",4));
/// assert_eq!(Some("今日は".to_string()),left("今日は晴です",3));
///```
pub fn left(target: impl OptionalAsRefStr, len: usize) -> Option<String> {
    match target.as_opt_str() {
        Some(value) => {
            if len == 0 {
                return Some(gen_empstring!());
            }

            if len > value.count_chars() {
                return Some(value.to_string());
            }
            Some(substring(value, 0, len))
        }
        None => None,
    }
}

/// ## Summary
///
/// ## Parameters
/// - `target`:
/// - `len`:
///
/// ## Returns
///
/// ## Examples
///```
/// use stringutils::stringutils::*;
///
/// let opt:Option<String> = None;
///
/// assert_eq!(None,right(opt,1));
/// assert_eq!(Some("".to_string()),right("abc",0));
/// assert_eq!(Some("bc".to_string()),right("abc",2));
/// assert_eq!(Some("abc".to_string()),right("abc",4));
/// assert_eq!(Some("晴です".to_string()),right("今日は晴です",3));
///```
pub fn right(target: impl OptionalAsRefStr, len: usize) -> Option<String> {
    match target.as_opt_str() {
        Some(value) => {
            if len == 0 {
                return Some(gen_empstring!());
            }

            let chars_len = value.count_chars();
            if len > chars_len {
                return Some(value.to_string());
            }

            Some(substring(value, chars_len - len, chars_len))
        }
        None => None,
    }
}

/// ## Summary
///
/// ## Parameters
/// - `target`:
/// - `pos`:
/// - `len`:
///
/// ## Returns
///
/// ## Examples
///```
/// use stringutils::stringutils::*;
///
/// let opt:Option<String> = None;
///
/// assert_eq!(None,mid(opt,1,2));
/// assert_eq!(Some("ab".to_string()),mid("abc",0,2));
/// assert_eq!(Some("abc".to_string()),mid("abc",0,4));
/// assert_eq!(Some("c".to_string()),mid("abc",2,4));
/// assert_eq!(Some("".to_string()),mid("abc",4,2));
/// assert_eq!(Some("日は雨".to_string()),mid("今日は雨です",1,2));
///```
pub fn mid(target: impl OptionalAsRefStr, pos: usize, len: usize) -> Option<String> {
    match target.as_opt_str() {
        Some(value) => {
            let char_count = value.count_chars();
            if pos >= char_count {
                return Some(gen_empstring!());
            }

            let end = pos + len;
            if char_count <= end {
                return Some(substring(target, pos, char_count));
            }
            Some(substring(target, pos, end))
        }
        None => None,
    }
}

pub fn starts_with_ignore_case(target: impl OptionalAsRefStr, prefix: impl AsRef<str>) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut target_chars = value.chars();
            for prefix_char in prefix.as_ref().chars() {
                let target_char = match target_chars.next() {
                    Some(c) => c,
                    None => return false,
                };

                if !target_char.to_lowercase().eq(prefix_char.to_lowercase()) {
                    return false;
                }
            }

            true
        }
        None => false,
    }
}

pub fn ends_with_ignore_case(target: impl OptionalAsRefStr, suffix: impl AsRef<str>) -> bool {
    match target.as_opt_str() {
        Some(value) => {
            let mut target_chars = value.chars().rev();
            for suffix_char in suffix.as_ref().chars().rev() {
                let target_char = match target_chars.next() {
                    Some(c) => c,
                    None => return false,
                };

                if !target_char.to_lowercase().eq(suffix_char.to_lowercase()) {
                    return false;
                }
            }

            true
        }
        None => false,
    }
}

/// 改行コード(\r,\n)を文字列の末尾から削除します
///
/// # Args
/// 対象文字列(&str,String,Option<&str>,Option<String>)
///
/// # Return
/// 末尾の改行コードを取り除いた文字列
///
/// # Examples
/// ```
/// use stringutils::stringutils::*;
///
/// let opt:Option<&str> = None;
///
/// assert_eq!("",chomp(opt));
/// assert_eq!("",chomp(""));
/// assert_eq!("zxy ",chomp("zxy \r"));
/// assert_eq!("zxy",chomp("zxy\n"));
/// assert_eq!("zxy",chomp("zxy\r\n"));
/// assert_eq!("zxy\r\n",chomp("zxy\r\n\r\n"));
/// assert_eq!("zxy\n",chomp("zxy\n\r"));
/// assert_eq!("zxy\n\rabc",chomp("zxy\n\rabc"));
/// assert_eq!("今日は曇りです",chomp("今日は曇りです\r\n"));
/// assert_eq!("",chomp("\r"));
/// assert_eq!("",chomp("\n"));
/// assert_eq!("",chomp("\r\n"));
/// ```
pub fn chomp(target: impl OptionalAsRefStr) -> String {
    match target.as_opt_str() {
        Some(value) => {
            let len = value.count_chars();
            if is_empty(value) {
                return value.to_string();
            }

            if len == 1 {
                let ch = value.chars().nth(0).unwrap();
                if ch == charutils::CR || ch == charutils::LF {
                    return gen_empstring!();
                }
                return value.to_string();
            }

            let mut last_idx = len.saturating_sub(1);

            if let Some(last_char) = value.chars().nth(last_idx) {
                if last_char == charutils::LF {
                    if value.chars().nth(last_idx - 1).unwrap() == charutils::CR {
                        last_idx -= 1;
                    }
                } else if last_char != charutils::CR {
                    last_idx += 1;
                }
            }

            substring(target, 0, last_idx)
        }
        None => gen_empstring!(),
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
        println!("{}", abbreviate("This is a very long text message.", 2));
        println!("{}", swap_case("Hello World!23"));
        println!("{:?}", substring_before("user@example.com", "@"));
        println!("{:?}", substring_after("user@example.com", "@"));
        println!("{:?}", substring_after("test@", "@"));
        println!("{}", delete_whitespace(" fas  d d s a f "));
        println!("{:?}", left("abcdef", 3));
        println!("{:?}", left("abc", 5));
        println!("{:?}", right("abcdef", 3));
        println!("{:?}", right("abc", 5));
        println!("{:?}", mid("abcdef", 2, 3));
        println!("{:?}", mid("abcdef", 4, 10));

        println!("{}", starts_with_ignore_case("Abcdef", "abc"));
        println!("{}", starts_with_ignore_case("Abcdef", "DEF"));
        println!("{}", ends_with_ignore_case("Abcdef", "DEF"));
        println!("{}", ends_with_ignore_case("Abcdef", "abc"));
    }
}
