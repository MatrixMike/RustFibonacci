fn main() {
    match fibonacci(8) {
        Ok(number) => println!("8th index number is {:?}", number),
        Err(mut error) => println!("error is {:?}", error.description()),
    }

    let before_overflow = fibonacci(85);
    match before_overflow {
        Ok(number) => println!("85th index number is {:?}", number),
        Err(mut error) => println!("error is {:?}", error.description()),
    }

    let overflow = fibonacci(10000);
    match overflow {
        Ok(number) => println!("10000th index number is {:?}", number),
        Err(mut error) => println!("error is {:?}", error.description()),
    }
}

enum FibonacciError {
    Overflow
}

impl FibonacciError {
    fn description(&mut self) -> String {
        match *self {
            FibonacciError::Overflow => return String::from("Overflow")
        };
    }
}

fn fibonacci(location: i64) -> Result<i64, FibonacciError> {
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
