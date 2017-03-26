mod strutil;

/// Generic type for Error.
///
/// The `Error` type can be converted from any types that implement
/// `std::fmt::Debug`. This enables to use `?` operator for any error type, and
/// it should be useful especially in writing scratch code.
///
/// ```
/// use easyutils;
///
/// fn example() -> Result<(), easyutils::Error> {
///   // parse returns Result<i32, FromStr::Err>.
///   let a = "12345".parse::<i32>()?;
///   println!("{}\n", a);
///   Ok(())
/// }
/// ```
pub struct Error(String);

impl<T: std::fmt::Debug> From<T> for Error {
    fn from(error: T) -> Self {
        Error(format!("{:?}", error))
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_error() {
        let result = || -> Result<(), Error> {
            "foo".parse::<i32>()?;
            Ok(())
        }();
        assert_eq!(match result {
                       Err(Error(s)) => s,
                       _ => "".into(),
                   },
                   "ParseIntError { kind: InvalidDigit }");
    }
}
