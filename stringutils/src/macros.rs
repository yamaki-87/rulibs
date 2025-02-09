/// 空文字のStringを生成
///
/// # Return
/// 空文字のString
///
/// # Examples
/// ```
/// let mut string = gen_empstring!();
/// assert_eq!("",string)
/// ```
#[macro_export]
macro_rules! gen_empstring {
    () => {
        "".to_string()
    };
}

/// literal(可変引数)を区切り文字で連結して１つの文字列にする
///
/// # Args
/// - `seperator`: 区切り文字
/// - `item`: リテラル(可変引数)
///
/// # Return
/// 結合された文字列
///
/// # Example
/// ```
/// let s = join!(",", "rust", "go");
/// assert_eq!("rust,go",s);
/// ```
#[macro_export]
macro_rules! join {
    ($seperator:expr,$($item:expr),*) => {
        vec![$($item),*].join($seperator)
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gen_empstring() {
        let emp_str = gen_empstring!();
        assert_eq!("", emp_str);
        assert_ne!("test", emp_str);
    }

    #[test]
    fn gen_join() {
        let s = join!(",", "test", "kon");
        assert_eq!("", s);
    }
}
