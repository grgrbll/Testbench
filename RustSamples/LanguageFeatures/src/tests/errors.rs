use std::fmt;


type Result<T> = std::result::Result<T, CustomError>;
struct CustomError;
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom Error Message")

    }
}

fn retError(b : bool) -> Result<i32> {
    if b { return Err(CustomError); }
    Ok(42)
}

#[test]
fn customError() {
    retError(true)


}