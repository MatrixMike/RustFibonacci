pub enum FibonacciError {
    Overflow
}

impl FibonacciError {
    pub fn description(&mut self) -> String {
        match *self {
            FibonacciError::Overflow => return String::from("Overflow")
        };
    }
}

pub fn fibonacci(location: i64) -> Result<i64, FibonacciError> {
    if location <= 2 {
        return Ok(1);
    }

    let mut first: i64 = 1;
    let mut second: i64 = 1;
    let mut index = 2;

    while index < location {
        match first.checked_add(second) {
            Some(number) => {
                first = second;
                second = number;
                index = index + 1;
            }
            None => {
                return Err(FibonacciError::Overflow)
            }
        };
    }

    Ok(second)
}
