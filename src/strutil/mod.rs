use std;

const TO_LOWER_HEX: &'static [char] = &['0', '1', '2', '3', '4', '5', '6',
                                        '7', '8', '9', 'a', 'b', 'c', 'd',
                                        'e', 'f'];
const TO_UPPER_HEX: &'static [char] = &['0', '1', '2', '3', '4', '5', '6',
                                        '7', '8', '9', 'A', 'B', 'C', 'D',
                                        'E', 'F'];

pub trait BinaryUtil {
    /// Converts a string into hexadecimal representation.
    ///
    /// ```
    /// use easyutils::*;
    ///
    /// assert_eq!("bar".as_bytes().bin2hex(), "626172");
    /// ```
    fn bin2hex(&self) -> String;

    /// URL-encodes a string.
    ///
    /// ```
    /// use easyutils::*;
    ///
    /// assert_eq!("abc def/ghi%jkl".as_bytes().urlencode(),
    ///            "abc+def%2Fghi%25jkl");
    /// ```
    fn urlencode(&self) -> String;
}

impl BinaryUtil for [u8] {
    fn bin2hex(&self) -> String {
        let mut result = String::with_capacity(self.len() * 2);
        for c in self {
            result.push(TO_LOWER_HEX[*c as usize / 16]);
            result.push(TO_LOWER_HEX[*c as usize % 16]);
        }
        result
    }

    fn urlencode(&self) -> String {
        // Reserve a string with minimal capacity.  The return value's length
        // should be longer than the original string's length.
        let mut result = String::with_capacity(self.len());
        for c in self {
            if (b'0' <= *c && *c <= b'9') || (b'a' <= *c && *c <= b'z') ||
               (b'A' <= *c && *c <= b'Z') || *c == b'-' ||
               *c == b'_' || *c == b'.' {
                result.push(*c as char);
            } else if *c == b' ' {
                result.push('+');
            } else {
                result.push('%');
                result.push(TO_UPPER_HEX[*c as usize / 16]);
                result.push(TO_UPPER_HEX[*c as usize % 16]);
            }
        }
        result
    }
}

pub trait StrUtil {
    /// Converts a string into hexadecimal representation.
    ///
    /// ```
    /// use easyutils::*;
    ///
    /// assert_eq!("bar".bin2hex(), "626172");
    /// ```
    fn bin2hex(&self) -> String;

    /// URL-encodes a string.
    ///
    /// ```
    /// use easyutils::*;
    ///
    /// assert_eq!("abc def/ghi%jkl".as_bytes().urlencode(),
    ///            "abc+def%2Fghi%25jkl");
    /// ```
    fn urlencode(&self) -> String;
}

impl<T: std::borrow::Borrow<str>> StrUtil for T {
    fn bin2hex(&self) -> String {
        self.borrow().as_bytes().bin2hex()
    }

    fn urlencode(&self) -> String {
        self.borrow().as_bytes().urlencode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin2hex() {
        assert_eq!("foo".bin2hex(), "666f6f");
        assert_eq!("foo".to_string().bin2hex(), "666f6f");
        assert_eq!("foo".as_bytes().bin2hex(), "666f6f");

        assert_eq!("".bin2hex(), "");
        assert_eq!("\x00\x01\x02\x7f".bin2hex(), "0001027f");
    }

    #[test]
    fn test_urlencode() {
        assert_eq!("abc def/ghi%jkl".urlencode(), "abc+def%2Fghi%25jkl");
        assert_eq!("abc def/ghi%jkl".as_bytes().urlencode(),
                   "abc+def%2Fghi%25jkl");

        assert_eq!("日本語".urlencode(), "%E6%97%A5%E6%9C%AC%E8%AA%9E");
        assert_eq!("\x00\x01\x02\x7f".urlencode(), "%00%01%02%7F");
        assert_eq!("!\"#$%&'()*+,-./".urlencode(),
                   "%21%22%23%24%25%26%27%28%29%2A%2B%2C-.%2F");
        assert_eq!(":;<=>?@[\\]^_`{|}~".urlencode(),
                   "%3A%3B%3C%3D%3E%3F%40%5B%5C%5D%5E_%60%7B%7C%7D%7E");
    }
}
